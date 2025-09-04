use chrono::{DateTime, Duration, Local};
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateBlock {
    None,
    InProgress,
    Paused,
    Finished,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeBlock {
    pub name: String,
    pub durantion: TimeRegister,
    pub message: Option<String>,
    pub start_time: TimeRegister,
    pub end_time: TimeRegister,
    pub state: StateBlock,

    pub denied_acess: Vec<String>,
    pub allow_acess: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TimeRegister {
    pub hours: u8,
    pub minutes: u8,
}

impl Serialize for TimeRegister {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let time_string = format!("{}:{}", self.hours, self.minutes);
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

        let hours: u8 = hours_str
            .parse()
            .map_err(|_| de::Error::custom(format!("hours not a valid number")))?;
        let minutes: u8 = minutes_str
            .parse()
            .map_err(|_| de::Error::custom(format!("minutes not a valid number")))?;

        if hours > 23 || minutes > 59 {
            return Err(de::Error::custom(format!("invalid time")));
        }

        Ok(TimeRegister { hours, minutes })
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
