mod dexcom;

use std::fs::File;

use anyhow::Result;
use clap::Parser;
use csv::ReaderBuilder;

use crate::dexcom::*;
use crate::dexcom::DexcomRecord::EGV;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CSV file to parse
    input_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_reader(File::open(args.input_file)?);

    let mut records: Vec<DexcomRecord> = Vec::new();

    for result in rdr.records() {
        let row = result?;
        let record = DexcomRecord::new(&row)?;
        records.push(record);
    }

    let egvs: Vec<&DexcomRecord> = records.iter()
        .filter(|r| matches!(r, EGV {..})).collect();
    for rec in egvs {
        println!("{:?}", rec);
    }
    Ok(())
}
