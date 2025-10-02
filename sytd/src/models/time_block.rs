use super::{StateBlock, TimeRegister};
use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeBlock {
    pub name: String,
    pub message: Option<String>,
    pub duration: TimeRegister,
    pub start_time: TimeRegister,
    pub end_time: TimeRegister,
    pub state: StateBlock,

    pub days: HashSet<Weekday>,

    pub denied_acess: Vec<String>,
    pub allow_acess: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,
}

impl TimeBlock {
    pub fn new() -> TimeBlockBuilder {
        TimeBlockBuilder::new()
    }
}

pub struct TimeBlockBuilder {
    time_block: TimeBlock,
    set_duration: bool,
    set_start_time: bool,
    set_end_time: bool,
}

impl TimeBlockBuilder {
    fn new() -> Self {
        Self {
            time_block: TimeBlock {
                name: String::new(),
                message: None,
                duration: TimeRegister::new(0, 0).unwrap(),
                start_time: TimeRegister::new(0, 0).unwrap(),
                end_time: TimeRegister::new(0, 0).unwrap(),
                state: StateBlock::Idle,
                days: HashSet::new(),
                denied_acess: vec![],
                allow_acess: vec![],
                denied_apps: vec![],
                allow_apps: vec![],
            },
            set_duration: false,
            set_start_time: false,
            set_end_time: false,
        }
    }

    pub fn name(&mut self, name: String) {
        self.time_block.name = name;
    }

    pub fn message(&mut self, message: Option<String>) {
        self.time_block.message = message;
    }

    pub fn duration(&mut self, duration: Option<TimeRegister>) {
        if let Some(time) = duration {
            self.time_block.duration = time;
            self.time_block.start_time = TimeRegister::new(0, 0).unwrap();
            self.time_block.end_time = TimeRegister::new(0, 0).unwrap();
            self.set_duration = true;
        } else {
            self.time_block.duration = TimeRegister::new(0, 0).unwrap();
            self.set_duration = false;
        }
    }

    pub fn time(&mut self, start_time: Option<TimeRegister>, end_time: Option<TimeRegister>) {
        self.set_start_time = start_time.is_some();
        self.set_end_time = end_time.is_some();

        self.time_block.start_time = start_time.unwrap_or(TimeRegister::new(0, 0).unwrap());
        self.time_block.end_time = end_time.unwrap_or(TimeRegister::new(0, 0).unwrap());
    }

    pub fn allow(&mut self, allow_acess: Vec<String>, allow_apps: Vec<String>) {
        self.time_block.allow_acess = allow_acess;
        self.time_block.allow_apps = allow_apps;
    }

    pub fn denied(&mut self, denied_acess: Vec<String>, denied_apps: Vec<String>) {
        self.time_block.denied_acess = denied_acess;
        self.time_block.denied_apps = denied_apps;
    }

    pub fn days(&mut self, days: Vec<Weekday>) {
        for day in days {
            self.time_block.days.insert(day);
        }
    }

    pub fn build(self) -> Result<TimeBlock, String> {
        if self.set_duration && (self.set_start_time || self.set_end_time) {
            return Err(String::from(
                "You cannot set a duration along with a start time or an end time in a time block.",
            ));
        } else if !(!self.set_duration && (self.set_start_time && self.set_end_time)) {
            return Err(String::from(
                "You need to define a start time and an end time.",
            ));
        }

        Ok(self.time_block)
    }
}
