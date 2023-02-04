use anyhow::Result;
use chrono::NaiveDateTime;
use csv::StringRecord;

use crate::dexcom::DexcomRecord::{Alert, Calibration, Carbs, Device, EGV, FirstName, Insulin, LastName, Unknown};

#[derive(Debug)]
pub(crate) enum DexcomRecord {
    FirstName { index: u32, patient_info: String },
    LastName { index: u32, patient_info: String },
    Device { index: u32, device_info: String, source_device_id: String },
    Alert {
        index: u32,
        event_subtype: String,
        source_device_id: String,
        glucose_value: Option<u16>,
        duration: Option<String>,
        glucose_rate_of_change: Option<u8>,
    },
    EGV {
        index: u32,
        timestamp: NaiveDateTime,
        source_device_id: String,
        glucose_value: u16,
        transmitter_time: u64,
        transmitter_id: String,
    },
    Insulin {
        index: u32,
        timestamp: NaiveDateTime,
        event_subtype: String,
        source_device_id: String,
        insulin_value: f64,
    },
    Carbs {
        index: u32,
        timestamp: NaiveDateTime,
        source_device_id: String,
        carb_value: u16,
    },
    Calibration {
        index: u32,
        timestamp: NaiveDateTime,
        source_device_id: String,
        glucose_value: u16,
        transmitter_id: String,
    },
    Unknown { index: u32 },
}

impl DexcomRecord {
    pub(crate) fn new(data: &StringRecord) -> Result<Self> {
        Ok(match &data[2] {
            "FirstName" => FirstName { index: data[0].parse()?, patient_info: data[4].to_string() },
            "LastName" => LastName { index: data[0].parse()?, patient_info: data[4].to_string() },
            "Device" => Device {
                index: data[0].parse()?,
                device_info: data[5].to_string(),
                source_device_id: data[6].to_string(),
            },
            "Alert" => Alert {
                index: data[0].parse()?,
                event_subtype: data[3].to_string(),
                source_device_id: data[6].to_string(),
                glucose_value: data[7].parse().ok(),
                duration: Some(data[10].to_string()).filter(|s| !s.is_empty()),
                glucose_rate_of_change: data[11].parse().ok(),
            },
            "EGV" => EGV {
                index: data[0].parse()?,
                timestamp: NaiveDateTime::parse_from_str(&data[1], "%Y-%m-%dT%H:%M:%S")?,
                source_device_id: data[6].to_string(),
                glucose_value: data[7].parse()?,
                transmitter_time: data[12].parse()?,
                transmitter_id: data[13].to_string(),
            },
            "Insulin" => Insulin {
                index: data[0].parse()?,
                timestamp: NaiveDateTime::parse_from_str(&data[1], "%Y-%m-%dT%H:%M:%S")?,
                event_subtype: data[3].to_string(),
                source_device_id: data[6].to_string(),
                insulin_value: data[8].parse()?,
            },
            "Carbs" => Carbs {
                index: data[0].parse()?,
                timestamp: NaiveDateTime::parse_from_str(&data[1], "%Y-%m-%dT%H:%M:%S")?,
                source_device_id: data[6].to_string(),
                carb_value: data[9].parse()?,
            },
            "Calibration" => Calibration {
                index: data[0].parse()?,
                timestamp: NaiveDateTime::parse_from_str(&data[1], "%Y-%m-%dT%H:%M:%S")?,
                source_device_id: data[6].to_string(),
                glucose_value: data[7].parse()?,
                transmitter_id: data[13].to_string(),
            },
            _ => Unknown { index: data[0].parse()? },
        })
    }
}
