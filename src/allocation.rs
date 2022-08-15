use crate::traits::TimeBound;
use chrono::{prelude::*, Duration};
use color_eyre::eyre::Result;

/// # Allocation
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Allocation {
    pub end_date: Date<Utc>,
    pub start_date: Date<Utc>,
}

impl Allocation {
    /// Returns the Allocation's duration.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use hallo::allocation::Allocation;
    ///
    /// let start = Utc.ymd(2014, 7, 8);
    /// let end = Utc.ymd(2014, 7, 10);
    /// let duration = Duration::days(2);
    /// let a = Allocation { start_date: start, end_date: end };
    /// assert_eq!(a.duration(), duration)
    /// ```
    pub fn duration(&self) -> Duration {
        self.end_date - self.start_date
    }

    /// Checks if an allocation is active for a given date.
    /// ```
    /// use chrono::prelude::*;
    /// use hallo::allocation::Allocation;
    ///
    /// let start = Utc.ymd(2014, 7, 8);
    /// let end = Utc.ymd(2014, 7, 10);
    /// let a = Allocation { start_date: start, end_date: end };
    /// assert_eq!(a.is_active_on(&Utc.ymd(2014, 7, 7)), false);
    /// assert_eq!(a.is_active_on(&Utc.ymd(2014, 7, 9)), true);
    /// assert_eq!(a.is_active_on(&Utc.ymd(2014, 7, 11)), false);
    /// ```
    pub fn is_active_on(&self, search_date: &Date<Utc>) -> bool {
        let search_too_early = *search_date <= self.start_date;
        if search_too_early {
            return false;
        }
        let search_too_late = *search_date > self.end_date;
        if search_too_late {
            return false;
        }
        true
    }
}

/// Returns
/// Note: all values are designed to be approximate.
impl Default for Allocation {
    fn default() -> Self {
        let start = Utc::today() + Duration::weeks(3);
        let end = start + Duration::weeks(4);
        Allocation {
            start_date: start,
            end_date: end,
        }
    }
}

impl std::fmt::Display for Allocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.start_date(), self.end_date())
    }
}

impl TimeBound for Allocation {
    fn start_date(&self) -> &Date<Utc> {
        &self.start_date
    }
    fn end_date(&self) -> &Date<Utc> {
        &self.end_date
    }
    fn set_start_date(
        &mut self,
        date: &Date<Utc>,
    ) -> Result<Date<Utc>, crate::traits::TimeBoundError> {
        self.start_date = *date;
        Ok(self.start_date)
    }
    fn set_end_date(
        &mut self,
        date: &Date<Utc>,
    ) -> Result<Date<Utc>, crate::traits::TimeBoundError> {
        self.end_date = *date;
        Ok(self.end_date)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn something() {
        assert!(true)
    }
}
