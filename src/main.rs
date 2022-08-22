use std::collections::BTreeMap;

use chrono::{Local, TimeZone};
use clap::Parser;
use inquire::validator::Validation;
use inquire::{CustomType, DateSelect, Text};
use prt::cli::Args;
use prt::model::{OpenPrediction, Predictions, Record};

use anyhow::Result;
fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        prt::cli::Command::Predict => predict(),
        prt::cli::Command::Resolve => resolve(),
        prt::cli::Command::Tally => tally(),
        prt::cli::Command::Show => show(),
    }?;

    Ok(())
}

fn predict() -> Result<()> {
    let statement = Text::new("Statement:")
        .with_validator(&|val: &str| {
            if val.is_empty() {
                Ok(Validation::Invalid("Please enter a statement".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;
    let probability_prompt: CustomType<f64> = CustomType {
        message: "Probability:",
        default: None,
        placeholder: Some("0.5"),
        help_message: "Enter as a decimal number between 0.0 and 1.0".into(),
        formatter: &|i| format!("{:>3.1}%", i * 100.0),
        parser: &|i| match i.parse::<f64>() {
            Ok(val) => {
                if (0.0..=1.0).contains(&val) {
                    Ok(val)
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        },
        error_message: "Please type a valid probabilty".into(),
        render_config: inquire::ui::RenderConfig::default_colored(),
    };
    let probability = probability_prompt.prompt()?;
    let resolves_after = DateSelect::new("Resolution Date:").prompt()?;
    let mut predictions = Predictions::read()?;
    predictions.open.push(OpenPrediction {
        record: Record {
            statement,
            probability,
            resolves_after: chrono::DateTime::<Local>::from_local(
                resolves_after.and_hms(0, 0, 0),
                Local
                    .offset_from_local_datetime(&Local::now().naive_local())
                    .unwrap(),
            ),
            created_on: Local::now(),
        },
    });
    predictions.write()?;
    Ok(())
}

fn resolve() -> Result<()> {
    let mut predictions = Predictions::read()?;
    let now = Local::now();

    fn make_resolve_prompt() -> CustomType<'static, bool> {
        CustomType {
            message: "How did this prediction resolve?",
            default: None,
            placeholder: Some("true/false"),
            help_message: Some("Press ESC to skip resolving"),
            formatter: &|val| format!("{val}"),
            parser: &|s| {
                if ["true", "t", "yes", "y", "0"].contains(&s.to_lowercase().as_str()) {
                    Ok(true)
                } else if ["false", "f", "no", "n", "1"].contains(&s.to_lowercase().as_str()) {
                    Ok(false)
                } else {
                    Err(())
                }
            },
            error_message: "Please enter a truthy value".into(),
            render_config: inquire::ui::RenderConfig::default_colored(),
        }
    }
    // ugly and potentially inefficient but until drain_filter becomes stable it will have to do
    let mut i = 0;
    while i < predictions.open.len() {
        let prediction = &predictions.open[i];
        if prediction.record.resolves_after <= now {
            println!("{prediction}");
            let resolution = make_resolve_prompt().prompt_skippable()?;
            match resolution {
                Some(resolution) => {
                    let prediction = predictions.open.remove(i);
                    predictions.resolved.push(prediction.resolve(resolution))
                }
                None => i += 1,
            }
        } else {
            i += 1;
        }
    }
    predictions.write()?;
    Ok(())
}

fn tally() -> Result<()> {
    let predictions = Predictions::read()?;

    // BTreeMaps are probably more efficient than HashMaps; but f64 doesn't implement Ord.
    // Since we are tracking bins anyways we will just use the value * 20 as an integer key
    // This gives us .05 sized bins
    let mut bins_correct = BTreeMap::<u32, u32>::new();
    let mut bins_incorrect = BTreeMap::<u32, u32>::new();
    fn to_bin(p: f64) -> u32 {
        (p * 20.).floor() as u32
    }
    fn from_bin(p: u32) -> f64 {
        p as f64 / 20.
    }
    for prediction in predictions.resolved {
        let p = prediction.record.probability;
        let correct = prediction.is_correct();
        let (p, correct) = if p >= 0.5 {
            (p, correct)
        } else {
            (1. - p, !correct)
        };
        if correct {
            *bins_correct.entry(to_bin(p)).or_default() += 1;
        } else {
            *bins_incorrect.entry(to_bin(p)).or_default() += 1;
        }
    }
    for i in 10..20 {
        let correct = bins_correct.entry(i).or_default();
        let incorrect = bins_incorrect.entry(i).or_default();
        println!(
            "{:>3.2} to {:>3.2}: {} correct and {} incorrect: {}",
            from_bin(i),
            from_bin(i + 1),
            correct,
            incorrect,
            if *correct + *incorrect > 0 {
                format!(
                    "{:5.1}% ",
                    *correct as f64 / (*correct + *incorrect) as f64 * 100.
                )
            } else {
                "  ---  ".into()
            }
        );
    }
    Ok(())
}

fn show() -> Result<()> {
    println!("{}", Predictions::read()?);
    Ok(())
}
