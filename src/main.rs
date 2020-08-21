use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

/// Input parameters will be passed to stdin in JSON format
#[derive(Deserialize)]
struct Input {
    pub isin: String,
}

/// Output to stdout. If the field 'error' is populated, or the return code from the process is
/// non-zero, then it will be assumed that the command failed. Otherwise,
#[derive(Serialize, Default)]
struct Output {
    pub results: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Record format of our (tab separated) security master file
#[derive(Debug, Deserialize)]
pub struct Record {
    pub ticker: String,
    pub issuer: String,
    pub issue: String,
    pub isin: String,
}

/// Look for a matching row in our security master
/// # Args
/// * `isin` - ISIN code to look up in the securities universe
///
fn lookup_ticker(isin: String) -> anyhow::Result<String> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/isins.tsv")?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.isin == isin {
            return Ok(record.ticker);
        }
    }

    Err(anyhow!(
        "Couldn't find a ticker corresponding to ISIN {}",
        isin
    ))
}

/// Read from stdin until EOF and then attempt to parse JSON into an Input struct.
fn read_input() -> Result<Input> {
    let mut json_in = String::new();

    io::stdin()
        .read_to_string(&mut json_in)
        .context("Failed to read JSON from stdin")?;
    let params = serde_json::from_str::<Input>(&json_in)
        .context("Failed to parse input message from JSON")?;

    Ok(params)
}

fn main() {
    match read_input().and_then(|input| lookup_ticker(input.isin)) {
        // For this simple example, we're mocking the existence of a 'real' securities universe.
        // We will simply take advantage of the fact that BBG and RIC codes are easily formed 
        // for ASX instruments (with the caveat that unlike the terminal, the BBG web interface expects a colon)
        Ok(ticker) => {
            let mut o = Output::default();
            o.results.insert("bbg_code".into(), format!("{}:AU", ticker));
            o.results.insert("ric_code".into(), format!("{}.AX", ticker));

            println!(
                "{}",
                serde_json::to_string(&o).expect("Failed to serialize output")
            );
        }
        Err(e) => {
            let o = serde_json::to_string(&Output {
                error: Some(format!("{:?}", e)),
                ..Default::default()
            })
            .expect("Failed to serialize output");
            println!("{:?}", o);
        }
    };
}
