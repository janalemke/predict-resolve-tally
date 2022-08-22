use std::fmt::Display;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Record {
    pub statement: String,
    pub probability: f64,
    pub resolves_after: DateTime<Local>,
    // metadata
    pub created_on: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct OpenPrediction {
    #[serde(flatten)]
    pub record: Record,
}

impl Display for OpenPrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Statement: {}", self.record.statement)?;
        writeln!(f, "Probability: {:>3.1}%", self.record.probability * 100.)?;
        writeln!(f, "Will resolve after: {}", self.record.resolves_after)?;
        Ok(())
    }
}

impl OpenPrediction {
    pub fn resolve(self, resolution: bool) -> ResolvedPrediction {
        ResolvedPrediction {
            record: self.record,
            resolved_to: resolution,
            resolved_on: Local::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ResolvedPrediction {
    #[serde(flatten)]
    pub record: Record,
    resolved_to: bool,
    resolved_on: DateTime<Local>,
}

impl Display for ResolvedPrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Statement: {}", self.record.statement)?;
        writeln!(f, "Probability: {:>3.1}%", self.record.probability * 100.)?;
        writeln!(f, "Resolved on: {}", self.resolved_on)?;
        writeln!(f, "Resolved: {}", self.resolved_to)?;
        Ok(())
    }
}

impl ResolvedPrediction {
    pub fn is_correct(&self) -> bool {
        self.resolved_to
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Predictions {
    pub open: Vec<OpenPrediction>,
    pub resolved: Vec<ResolvedPrediction>,
}

impl Display for Predictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Open predictions:")?;
        if self.open.is_empty() {
            writeln!(f, "None")?;
        } else {
            for open in &self.open {
                writeln!(f, "{open}")?;
            }
        }
        writeln!(f, "Resolved predictions:")?;
        if self.resolved.is_empty() {
            writeln!(f, "None")?;
        } else {
            for resolved in &self.resolved {
                writeln!(f, "{resolved}")?;
            }
        }
        Ok(())
    }
}

impl Predictions {
    pub fn read() -> Result<Self> {
        crate::data::read()
    }

    pub fn write(self) -> Result<()> {
        crate::data::write(self)
    }
}
