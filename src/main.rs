mod dexcom;

use std::fs::File;

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use csv::ReaderBuilder;
use itertools::Itertools;
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};
use plotly::layout::{Axis, Shape};
use plotly::layout::ShapeLayer::Below;
use plotly::layout::ShapeType::Rect;

use crate::dexcom::*;
use crate::dexcom::DexcomRecord::EGV;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CSV file to parse
    input_file: String,
    /// Date to graph data
    date: NaiveDate,
    /// Wake-up time for graph
    wake_time: NaiveTime,
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
    // for rec in egvs {
    //     println!("{:?} {:?}", rec.day(NaiveTime::from_hms_opt(4, 0, 0).unwrap()), rec);
    // }
    // for (day, recs) in &egvs.into_iter().group_by(|e| e.day(NaiveTime::from_hms_opt(4, 0, 0).unwrap())) {
    //     println!("{:?}: {}", day, recs.count());
    // }

    let data = egvs.clone().into_iter()
        .into_group_map_by(|x| x.day(args.wake_time));
    let x: Vec<NaiveDateTime> = data.get(&Some(args.date)).unwrap().iter()
        .map(|x| x.timestamp().unwrap().clone()).collect();
    let y: Vec<u16> = data.get(&Some(args.date)).unwrap().iter()
        .map(|x| x.glucose_value().unwrap()).collect();
    let trace = Scatter::new(x.clone(), y.clone()).mode(Mode::Markers);
    let layout = Layout::new()
        .y_axis(Axis::new().range(vec![0, 300]))
        .x_axis(Axis::new().range(vec![x.first().unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
                                       x.last().unwrap().format("%Y-%m-%d %H:%M:%S").to_string()]))
        .shapes(vec![
            Shape::new().layer(Below).x_ref("paper").x0(0).x1(1).y0(0).y1(69)
                .shape_type(Rect).fill_color("red").opacity(0.25),
            Shape::new().layer(Below).x_ref("paper").x0(0).x1(1).y0(70).y1(179)
                .shape_type(Rect).fill_color("green").opacity(0.25),
            Shape::new().layer(Below).x_ref("paper").x0(0).x1(1).y0(180).y1(249)
                .shape_type(Rect).fill_color("gray").opacity(0.25),
            Shape::new().layer(Below).x_ref("paper").x0(0).x1(1).y0(250).y1(300)
                .shape_type(Rect).fill_color("yellow").opacity(0.25),
        ]);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.write_html("plot.html");

    Ok(())
}
