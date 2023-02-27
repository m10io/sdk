use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use std::io::LineWriter;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Format {
    Json,
    Yaml,
    #[default]
    Raw,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "raw" => Ok(Format::Raw),
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
}

pub trait PrettyPrint {
    fn print(&self, format: Format) -> Result<(), PrintError>;
}

impl<Item: Serialize> PrettyPrint for Item {
    fn print(&self, format: Format) -> Result<(), PrintError> {
        match format {
            Format::Json => {
                let stdout = io::stdout();
                let handle = stdout.lock();
                let writer = LineWriter::new(handle);
                serde_json::to_writer_pretty(writer, self)?;
            }
            Format::Yaml => {
                let stdout = io::stdout();
                let handle = stdout.lock();
                let writer = LineWriter::new(handle);
                serde_yaml::to_writer(writer, self)?;
            }
            Format::Raw => {
                let pretty = PrettyConfig::new()
                    .depth_limit(4)
                    .separate_tuple_members(true);
                let s = to_string_pretty(self, pretty)?;
                println!("{}", s);
            }
        }
        Ok(())
    }
}
