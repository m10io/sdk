use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use std::io::LineWriter;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Json,
    Yaml,
    #[default]
    Raw,
    Csv,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "raw" => Ok(Format::Raw),
            "csv" => Ok(Format::Csv),
            _ => Err("no match"),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Format::Json => "json",
            Format::Yaml => "yaml",
            Format::Raw => "raw",
            Format::Csv => "csv",
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PrintError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Ron(#[from] ron::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
}

pub trait PrettyPrint {
    fn print(&self, format: Format) -> Result<(), PrintError>;
}

impl<Item: Serialize> PrettyPrint for Item {
    fn print(&self, format: Format) -> Result<(), PrintError> {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let writer = LineWriter::new(handle);
        match format {
            Format::Json => {
                serde_json::to_writer_pretty(writer, self)?;
            }
            Format::Yaml => {
                serde_yaml::to_writer(writer, self)?;
            }
            Format::Raw => {
                let pretty = PrettyConfig::new()
                    .depth_limit(4)
                    .separate_tuple_members(true)
                    .compact_arrays(true);
                ron::ser::to_writer_pretty(writer, self, pretty)?;
            }
            Format::Csv => {
                let mut output = csv::Writer::from_writer(writer);
                output.serialize(self)?;
            }
        }
        println!();
        Ok(())
    }
}
