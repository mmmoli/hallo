use chrono::{Date, Utc};
use color_eyre::eyre::Result;

pub trait Contribution {
    fn get_contribution_on(&self, date: &Date<Utc>) -> u32;
}

#[derive(Debug, PartialEq)]
pub enum TimeBoundError {
    InvalidDatesError,
}

impl std::error::Error for TimeBoundError {}
impl std::fmt::Display for TimeBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TimeBoundError::InvalidDatesError => {
                write!(f, "Start and end dates are invalid. Go double-check.")
            }
        }
    }
}

pub trait TimeBound {
    fn start_date(&self) -> &Date<Utc>;
    fn end_date(&self) -> &Date<Utc>;
    fn set_start_date(&mut self, date: &Date<Utc>) -> Result<Date<Utc>, TimeBoundError>;
    fn set_end_date(&mut self, date: &Date<Utc>) -> Result<Date<Utc>, TimeBoundError>;
}
