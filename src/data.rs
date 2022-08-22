use std::{fs::File, io::Read};

use crate::model::Predictions;
use anyhow::Result;
use directories::ProjectDirs;

fn get_dirs() -> ProjectDirs {
    ProjectDirs::from("de.jana-lemke", "", "predict-resolve-tally")
        .expect("Could not identify home directory of calling user")
}

fn open_prediction_file() -> Result<File> {
    // TODO: maybe allow other input formats
    let dir = get_dirs().data_dir().to_path_buf();
    let filepath = dir.join("predictions.json");
    std::fs::create_dir_all(dir)?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&filepath)?;
    Ok(file)
}

pub fn read() -> Result<Predictions> {
    let mut buf = String::new();
    open_prediction_file()?.read_to_string(&mut buf)?;
    if buf.is_empty() {
        Ok(Predictions::default())
    } else {
        Ok(serde_json::from_str(&buf)?)
    }
}
pub fn write(predictions: Predictions) -> Result<()> {
    //TODO: make this pretty print the json
    //TODO: also allow TOML input
    serde_json::to_writer_pretty(open_prediction_file()?, &predictions)?;
    Ok(())
}
