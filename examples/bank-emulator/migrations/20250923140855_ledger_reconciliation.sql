INSERT INTO
    public.bank_account_ranges (
        account_type,
        idx,
        max_number,
        last_used_number
    )
VALUES (
        'liability',
        600000000,
        99999999,
        0
    );

CREATE VIEW v_cla_by_currency AS
WITH accounts_with_contact AS (
  SELECT DISTINCT account
  FROM bank_contacts
)
SELECT
  ba.currency,
  COALESCE(SUM(ba.balance)::bigint, 0) AS cla_balance
FROM bank_accounts ba
JOIN accounts_with_contact awc ON awc.account = ba.id
WHERE ba.account_status IN ('open','pending')
  AND ba.account_type IN ('checking','savings')
GROUP BY ba.currency;