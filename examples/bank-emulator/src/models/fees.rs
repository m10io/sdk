#![allow(clippy::non_canonical_partial_ord_impl)]
#[cfg(test)]
use std::collections::HashMap;
use std::{cmp::Ordering, ops::RangeInclusive};

#[cfg(test)]
use m10_sdk::sdk::TransferStep;
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{types::Json, Executor, Postgres};
#[cfg(test)]
use tracing::warn;

use crate::{config::CurrencyConfig, error::Error};

const FEE_METADATA_KEY: &str = "fee_metadata";

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct FeeMetadata {
    #[serde(flatten)]
    pub schedule: FeeSchedule,
    pub split: Vec<FeeSplit>,
}

impl std::fmt::Display for FeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeType::Transfer => write!(f, "transfer"),
            FeeType::Withdraw => write!(f, "withdraw"),
        }
    }
}

impl FeeMetadata {
    pub async fn get(
        db: impl Executor<'_, Database = Postgres>,
        fee_type: FeeType,
        currency: &str,
    ) -> Result<Self, Error> {
        Metadata::get(
            db,
            &format!("{}_{}_{}", currency, fee_type, FEE_METADATA_KEY),
        )
        .await
    }

    pub async fn insert(
        &self,
        db: impl Executor<'_, Database = Postgres>,
        currency: &str,
        fee_type: FeeType,
    ) -> Result<(), Error> {
        Metadata {
            key: format!("{}_{}_{}", currency, fee_type, FEE_METADATA_KEY),
            value: Json(self.clone()),
        }
        .insert(db)
        .await
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        self.schedule.validate()?;
        if self.split.iter().map(|s| s.percent).sum::<Decimal>() != Decimal::new(1, 0) {
            return Err(Error::validation("split", "percents must add to 1"));
        }
        Ok(())
    }

    pub fn calculate(&self, amount: u64, currency: &CurrencyConfig) -> Result<FeeResponse, Error> {
        let bracket = self.schedule.bracket_by_amount(amount)?;
        let bracket_amount = currency.decimal_from_cents(bracket.amount(amount))?;
        let fees = self
            .split
            .iter()
            .map(|split| {
                Ok(Fee {
                    name: split.name.clone(),
                    amount: currency.cents_from_decimal(bracket_amount * split.percent)?,
                    account: split.account,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(FeeResponse { fees })
    }

    #[cfg(test)]
    pub fn validate_steps(
        &self,
        steps: Vec<TransferStep>,
        currency_config: &CurrencyConfig,
    ) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut freezable_accounts = vec![];
        let mut paid_fees: HashMap<_, u64> = self.split.iter().map(|s| (s.account, 0)).collect();
        let mut required_fees: HashMap<_, u64> =
            self.split.iter().map(|s| (s.account, 0)).collect();
        for step in steps.into_iter() {
            if let Some(fee) = paid_fees.get_mut(&step.to_account_id[..]) {
                *fee += step.amount;
            } else {
                freezable_accounts.push(step.from_account_id);
                let fee_resp = self.calculate(step.amount, currency_config)?;
                for required_fee in fee_resp.fees.iter() {
                    if let Some(fee) = required_fees.get_mut(&required_fee.account) {
                        *fee += required_fee.amount;
                    } else {
                        warn!("fee response mismatch");
                    }
                }
            }
        }
        if paid_fees != required_fees {
            warn!(?paid_fees, ?required_fees, "unpaid fees found");
            Ok(Some(freezable_accounts))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    Transfer,
    Withdraw,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct FeeSplit {
    pub name: String,
    pub percent: Decimal,
    #[serde(with = "hex")]
    pub account: [u8; 16],
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeeSchedule {
    pub fees: Vec<FeeBracket>,
}

impl FeeSchedule {
    pub fn validate(&mut self) -> Result<(), Error> {
        self.fees.sort();
        for (i, bracket) in self.fees.iter().enumerate() {
            if i > 0 {
                if let Some(left) = self.fees.get(i - 1) {
                    if *left.range.end() + 1 != *bracket.range.start() {
                        return Err(Error::validation("fees", "fees must be contigous"));
                    }
                }
            }
            if let Some(right) = self.fees.get(i + 1) {
                if bracket.range.end().checked_add(1).unwrap_or(u64::MAX) != *right.range.start() {
                    return Err(Error::validation("fees", "fees must be contiguous"));
                }
            }
        }
        let fee = self
            .fees
            .last()
            .ok_or_else(|| Error::validation("fees", "there must be at least one bucket"))?;
        if *fee.range.end() != u64::MAX {
            return Err(Error::validation("fees", "one bucket must end at u64::MAX"));
        }
        Ok(())
    }

    pub fn bracket_by_amount(&self, amount: u64) -> Result<&FeeBracket, Error> {
        self.fees
            .binary_search_by(|item| {
                if item.range.contains(&amount) {
                    return Ordering::Equal;
                }
                if *item.range.start() < amount {
                    return Ordering::Less;
                }
                Ordering::Greater
            })
            .ok()
            .and_then(|i| self.fees.get(i))
            .ok_or_else(|| Error::not_found("fee bracket"))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeeBracket {
    pub range: RangeInclusive<u64>,
    pub polynomial: Vec<Decimal>,
}

impl FeeBracket {
    fn amount(&self, amount: u64) -> u64 {
        // Execute the polynomial in the format of a*x + b*x + c*x^2 + ..
        self.polynomial
            .iter()
            .enumerate()
            .map(|(i, coef)| {
                (Decimal::new(u64::pow(amount, i as u32) as i64, 0) * coef)
                    .round()
                    .mantissa() as u64
            })
            .sum()
    }
}

impl PartialOrd for FeeBracket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.range == other.range {
            return Some(Ordering::Equal);
        }
        Some(self.range.start().cmp(other.range.end()))
    }
}

impl Ord for FeeBracket {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FeeResponse {
    pub fees: Vec<Fee>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Fee {
    pub name: String,
    pub amount: u64,
    #[serde(with = "hex")]
    pub account: [u8; 16],
}

#[derive(sqlx::FromRow)]
struct Metadata<T: DeserializeOwned + Serialize + Send + Sync> {
    key: String,
    value: Json<T>,
}

impl<T> Metadata<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + Unpin + 'static,
{
    async fn insert(&self, db: impl Executor<'_, Database = Postgres>) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO metadata (key, value) VALUES ($1, $2)
            ON CONFLICT (key) DO UPDATE SET value = $2 WHERE metadata.key = $1;",
        )
        .bind(&self.key)
        .bind(&self.value)
        .execute(db)
        .await?;
        Ok(())
    }

    async fn get(db: impl Executor<'_, Database = Postgres>, key: &str) -> Result<T, Error> {
        let metadata: Self = sqlx::query_as("SELECT * FROM metadata WHERE metadata.key = $1")
            .bind(key)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| Error::not_found("metadata"))?;
        Ok(metadata.value.0)
    }
}

#[cfg(test)]
mod tests {
    use m10_sdk::sdk::{self, TransferStep};
    use m10_sdk::Metadata;
    use rust_decimal::Decimal;

    use crate::{
        config::CurrencyConfig,
        models::{Fee, FeeBracket, FeeSchedule, FeeSplit},
    };

    use super::FeeMetadata;

    #[test]
    fn test_schedule_validation_failure() {
        let mut intersection = FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 2..=3,
                    polynomial: vec![Decimal::new(1, 0)],
                },
                FeeBracket {
                    range: 4..=u64::MAX,
                    polynomial: vec![Decimal::new(1, 0)],
                },
            ],
        };
        intersection
            .validate()
            .expect_err("intersecting fees passed");
        let mut intersection = FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 11..=20,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 2..=3,
                    polynomial: vec![Decimal::new(1, 0)],
                },
                FeeBracket {
                    range: 12..=u64::MAX,
                    polynomial: vec![Decimal::new(0, 0)],
                },
            ],
        };
        intersection
            .validate()
            .expect_err("intersecting fees passed");
        let mut gaps = FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 12..=20,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 25..=u64::MAX,
                    polynomial: vec![Decimal::new(0, 0)],
                },
            ],
        };
        gaps.validate().expect_err("schedule with gaps passed");
    }

    #[test]
    fn test_sorting() {
        let mut unsorted = FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 21..=u64::MAX,
                    polynomial: vec![Decimal::new(0, 0)],
                },
                FeeBracket {
                    range: 11..=20,
                    polynomial: vec![Decimal::new(0, 0)],
                },
            ],
        };
        unsorted.validate().expect("validation failed");
        assert_eq!(
            unsorted,
            FeeSchedule {
                fees: vec![
                    FeeBracket {
                        range: 0..=10,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 11..=20,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 21..=u64::MAX,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_calculate_fees() {
        let currency = CurrencyConfig::new_test();
        let metadata = FeeMetadata {
            schedule: FeeSchedule {
                fees: vec![
                    FeeBracket {
                        range: 0..=10,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 11..=20,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 21..=501,
                        polynomial: vec![Decimal::new(100, 0)],
                    },
                    FeeBracket {
                        range: 502..=u64::MAX,
                        polynomial: vec![Decimal::new(107, 0)],
                    },
                ],
            },
            split: vec![
                FeeSplit {
                    name: "M10".to_string(),
                    percent: Decimal::new(75, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                },
                FeeSplit {
                    name: "Acme".to_string(),
                    percent: Decimal::new(25, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                },
            ],
        };
        let res = metadata.calculate(500, &currency).expect("fee calc failed");
        assert_eq!(
            &res.fees[0],
            &Fee {
                name: "M10".to_string(),
                amount: 75,
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            }
        );
        assert_eq!(
            &res.fees[1],
            &Fee {
                name: "Acme".to_string(),
                amount: 25,
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            }
        );
        let res = metadata.calculate(555, &currency).expect("fee calc failed");
        assert_eq!(
            &res.fees[0],
            &Fee {
                name: "M10".to_string(),
                amount: 80,
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            }
        );
        assert_eq!(
            &res.fees[1],
            &Fee {
                name: "Acme".to_string(),
                amount: 27,
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            }
        );
    }

    #[test]
    fn test_validation_succeed() {
        let currency = CurrencyConfig::new_test();
        let metadata = FeeMetadata {
            schedule: FeeSchedule {
                fees: vec![
                    FeeBracket {
                        range: 0..=10,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 11..=20,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 21..=501,
                        polynomial: vec![Decimal::new(100, 0)],
                    },
                    FeeBracket {
                        range: 502..=u64::MAX,
                        polynomial: vec![Decimal::new(107, 0)],
                    },
                ],
            },
            split: vec![
                FeeSplit {
                    name: "M10".to_string(),
                    percent: Decimal::new(75, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                },
                FeeSplit {
                    name: "Acme".to_string(),
                    percent: Decimal::new(25, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                },
            ],
        };
        let res = metadata.calculate(500, &currency).expect("fee calc failed");
        let mut steps = vec![TransferStep {
            from_account_id: vec![1],
            to_account_id: vec![0],
            amount: 500,
            metadata: vec![m10_sdk::memo("main").any()],
        }];
        for fee in res.fees {
            steps.push(TransferStep {
                from_account_id: vec![1],
                to_account_id: fee.account.to_vec(),
                amount: fee.amount,
                metadata: vec![sdk::Fee {}.any()],
            });
        }
        let res = metadata
            .validate_steps(steps, &currency)
            .expect("fee calc failed");
        assert_eq!(res, None);
        let steps = vec![
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0],
                amount: 500,
                metadata: vec![m10_sdk::memo("main").any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                amount: 25,
                metadata: vec![sdk::Fee {}.any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                amount: 25,
                metadata: vec![sdk::Fee {}.any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                amount: 50,
                metadata: vec![sdk::Fee {}.any()],
            },
        ];
        let res = metadata
            .validate_steps(steps, &currency)
            .expect("fee calc failed");
        assert_eq!(res, None);
    }

    #[test]
    fn test_validation_failed() {
        let currency = CurrencyConfig::new_test();
        let metadata = FeeMetadata {
            schedule: FeeSchedule {
                fees: vec![
                    FeeBracket {
                        range: 0..=10,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 11..=20,
                        polynomial: vec![Decimal::new(0, 0)],
                    },
                    FeeBracket {
                        range: 21..=501,
                        polynomial: vec![Decimal::new(100, 0)],
                    },
                    FeeBracket {
                        range: 502..=u64::MAX,
                        polynomial: vec![Decimal::new(107, 0)],
                    },
                ],
            },
            split: vec![
                FeeSplit {
                    name: "M10".to_string(),
                    percent: Decimal::new(75, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                },
                FeeSplit {
                    name: "Acme".to_string(),
                    percent: Decimal::new(25, 2),
                    account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                },
            ],
        };
        let steps = vec![
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0],
                amount: 500,
                metadata: vec![m10_sdk::memo("main").any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                amount: 25,
                metadata: vec![sdk::Fee {}.any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                amount: 20,
                metadata: vec![sdk::Fee {}.any()],
            },
            TransferStep {
                from_account_id: vec![1],
                to_account_id: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
                amount: 50,
                metadata: vec![sdk::Fee {}.any()],
            },
        ];
        let res = metadata
            .validate_steps(steps, &currency)
            .expect("fee calc failed");
        assert_eq!(res, Some(vec![vec![1]]));
    }
}
