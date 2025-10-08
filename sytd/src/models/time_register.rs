use chrono::{DateTime, Local, TimeZone, Timelike};
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeRegister {
    local: DateTime<Local>,
}

impl TimeRegister {
    pub fn new(hours: u32, minutes: u32) -> Result<Self, String> {
        let date = Local::now().date_naive();
        let date_with_hours = date
            .and_hms_micro_opt(hours, minutes, 0, 0)
            .ok_or(String::from("invalid time"))?;

        Ok(Self {
            local: Local.from_local_datetime(&date_with_hours).unwrap(),
        })
    }

    pub fn from_local(date: DateTime<Local>) -> Self {
        Self { local: date }
    }
}

impl Serialize for TimeRegister {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let time_string = format!("{:02}:{:02}", self.local.hour(), self.local.minute());
        serializer.serialize_str(&time_string)
    }
}

struct TimeRegisterVisitor;
impl<'de> Visitor<'de> for TimeRegisterVisitor {
    type Value = TimeRegister;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expect a string in format: HH:mm - {12:00}")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (hours_str, minutes_str) = v
            .split_once(':')
            .ok_or_else(|| de::Error::custom(format!("Invalid format, expect format HH:mm")))?;

        let hours: u32 = hours_str
            .parse()
            .map_err(|_| de::Error::custom(format!("hours not a valid number")))?;
        let minutes: u32 = minutes_str
            .parse()
            .map_err(|_| de::Error::custom(format!("minutes not a valid number")))?;

        let date = Local::now().date_naive();
        let date_with_hours = date
            .and_hms_micro_opt(hours, minutes, 0, 0)
            .ok_or(de::Error::custom(format!("invalid time")))?;

        Ok(TimeRegister {
            local: Local.from_local_datetime(&date_with_hours).unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for TimeRegister {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeRegisterVisitor)
    }
}
