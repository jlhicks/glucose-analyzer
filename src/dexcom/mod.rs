use anyhow::Result;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use csv::StringRecord;

use crate::dexcom::DexcomRecord::{Alert, Calibration, Carbs, Device, EGV, FirstName, Insulin, LastName, Unknown};

#[derive(Debug)]
pub enum DexcomRecord {
    FirstName { index: u32, patient_info: String },
    LastName { index: u32, patient_info: String },
    Device { index: u32, device_info: String, source_device_id: String },
    Alert {
        index: u32,
        event_subtype: String,
        source_device_id: String,
        glucose_value: Option<u16>,
        duration: Option<Duration>,
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
    pub fn new(data: &StringRecord) -> Result<Self> {
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
                duration: match NaiveTime::parse_from_str(&data[10], "%H:%M:%S") {
                    Ok(t) => Some(t - NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                    _ => None,
                },
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

    pub fn day(&self, wake_up: NaiveTime) -> Option<NaiveDate> {
        match self {
            EGV { timestamp, .. } |
            Insulin { timestamp, .. } |
            Carbs { timestamp, .. } |
            Calibration { timestamp, .. } =>
                Some((*timestamp -
                    (wake_up - NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
                    .date()),
            _ => None,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            FirstName { index, .. } |
            LastName { index, .. } |
            Device { index, .. } |
            Alert { index, .. } |
            EGV { index, .. } |
            Insulin { index, .. } |
            Carbs { index, .. } |
            Calibration { index, .. } |
            Unknown { index, .. } => *index,
        }
    }

    pub fn patient_info(&self) -> Option<&str> {
        match self {
            FirstName { patient_info, .. } |
            LastName { patient_info, .. } => Some(patient_info),
            _ => None,
        }
    }

    pub fn device_info(&self) -> Option<&str> {
        match self {
            Device { device_info, .. } => Some(device_info),
            _ => None,
        }
    }

    pub fn source_device_id(&self) -> Option<&str> {
        match self {
            Device { source_device_id, .. } |
            Alert { source_device_id, .. } |
            EGV { source_device_id, .. } |
            Insulin { source_device_id, .. } |
            Carbs { source_device_id, .. } |
            Calibration { source_device_id, .. } => Some(source_device_id),
            _ => None,
        }
    }

    pub fn event_subtype(&self) -> Option<&str> {
        match self {
            Alert { event_subtype, .. } |
            Insulin { event_subtype, .. } => Some(event_subtype),
            _ => None,
        }
    }
    pub fn glucose_value(&self) -> Option<u16> {
        match self {
            Alert { glucose_value, .. } => *glucose_value,
            EGV { glucose_value, .. } |
            Calibration { glucose_value, .. } => Some(*glucose_value),
            _ => None,
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match self {
            Alert { duration, .. } => *duration,
            _ => None,
        }
    }
    pub fn glucose_rate_of_change(&self) -> Option<u8> {
        match self {
            Alert { glucose_rate_of_change, .. } => *glucose_rate_of_change,
            _ => None,
        }
    }

    pub fn timestamp(&self) -> Option<&NaiveDateTime> {
        match self {
            EGV { timestamp, .. } |
            Insulin { timestamp, .. } |
            Carbs { timestamp, .. } |
            Calibration { timestamp, .. } => Some(timestamp),
            _ => None,
        }
    }

    pub fn transmitter_time(&self) -> Option<u64> {
        match self {
            EGV { transmitter_time, .. } => Some(*transmitter_time),
            _ => None,
        }
    }

    pub fn transmitter_id(&self) -> Option<&str> {
        match self {
            EGV { transmitter_id, .. } |
            Calibration { transmitter_id, .. } => Some(transmitter_id),
            _ => None,
        }
    }

    pub fn insulin_value(&self) -> Option<f64> {
        match self {
            Insulin { insulin_value, .. } => Some(*insulin_value),
            _ => None,
        }
    }

    pub fn carb_value(&self) -> Option<u16> {
        match self {
            Carbs { carb_value, .. } => Some(*carb_value),
            _ => None,
        }
    }
}
