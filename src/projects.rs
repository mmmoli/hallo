use crate::{allocation::Allocation, traits::Contribution};
use chrono::{prelude::*, Duration};

#[derive(PartialEq, Debug)]
pub enum ProjectBuilderError {
    ZeroLengthDuration,
}

impl std::error::Error for ProjectBuilderError {}
impl std::fmt::Display for ProjectBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProjectBuilderError::ZeroLengthDuration => write!(f, "Project has no duration."),
        }
    }
}

/// # ProjectBuilder
/// Constructs Projects.
#[derive(PartialEq, Debug)]
pub struct ProjectBuilder {
    allocation: Allocation,
    name: String,
    value: u32,
}

impl Default for ProjectBuilder {
    fn default() -> Self {
        ProjectBuilder {
            allocation: Allocation::default(),
            name: "New Project".into(),
            value: 20000,
        }
    }
}

impl ProjectBuilder {
    /// This method sets the project's name.
    ///
    /// ## Example
    /// ```
    /// use hallo::projects::ProjectBuilder;
    /// let project = ProjectBuilder::default()
    ///   .name("My New Project".into())
    ///   .build();
    /// assert_eq!(project.name, "My New Project".to_string())
    /// ```
    pub fn name(mut self, name: String) -> ProjectBuilder {
        self.name = name;
        self
    }

    /// This method sets the project's duration.
    ///
    /// ## Example
    /// ```
    /// use hallo::projects::ProjectBuilder;
    /// use chrono::Duration;
    ///
    /// let duration = Duration::weeks(8);
    /// let project = ProjectBuilder::default()
    ///   .duration(&duration)
    ///   .build();
    /// assert_eq!(project.duration(), duration)
    /// ```
    pub fn duration(mut self, duration: &Duration) -> ProjectBuilder {
        let start_date = self.allocation.start_date;
        self.allocation = Allocation {
            start_date,
            end_date: start_date + *duration,
        };
        self
    }

    pub fn build(self) -> Project {
        Project {
            allocation: self.allocation,
            approx_value: self.value,
            name: self.name,
        }
    }
}

/// # Project
/// Represents a piece of work we might do in the future.
/// Note: all values are designed to be approximate.
#[derive(PartialEq, Debug)]
pub struct Project {
    allocation: Allocation,
    pub name: String,
    approx_value: u32,
}

/// Returns
/// Note: all values are designed to be approximate.
impl Default for Project {
    fn default() -> Self {
        Project {
            allocation: Allocation::default(),
            approx_value: 20000,
            name: "New Project".into(),
        }
    }
}

impl Project {
    /// Returns the Project's approximate duration.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use hallo::projects::Project;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(4);
    /// let approx_value: u32 = 20000;
    /// let p = Project::default();
    /// assert_eq!(p.duration(), duration)
    /// ```
    pub fn duration(&self) -> Duration {
        self.allocation.duration()
    }

    /// Returns the Project's approximate value.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use hallo::projects::Project;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::default();
    /// assert_eq!(p.value(), approx_value)
    /// ```
    pub fn value(&self) -> u32 {
        self.approx_value
    }
}

impl Contribution for Project {
    /// Returns the contribution for a given month.
    ///
    /// ### Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use hallo::projects::Project;    
    /// use hallo::traits::Contribution;    
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::default();
    /// p.get_contribution_on(&Utc.ymd(2014, 7, 8));
    /// ```    
    fn get_contribution_on(&self, date: &Date<Utc>) -> u32 {
        match self.allocation.is_active_on(date) {
            true => self.approx_value,
            false => 0 as u32,
        }
    }
}

#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn random_contribution() {
    //     let name = String::from("My Project");
    //     let start = Utc.ymd(2022, 7, 8);
    //     let duration = Duration::weeks(2);
    //     let approx_value: u32 = 20000;
    //     let p = Project::new(name, approx_value, start, &duration);
    //     let during = start + Duration::weeks(1);
    //     let contribution = p.get_contribution_on(during);
    //     assert!(contribution >= 0);
    //     assert!(contribution <= approx_value as i32)
    // }

    // #[test]
    // fn contribution_in_past() {
    //     let name = String::from("My Project");
    //     let start = Utc.ymd(2022, 7, 8);
    //     let duration = Duration::weeks(2);
    //     let approx_value: u32 = 20000;
    //     let p = Project::new(name, approx_value, start, &duration);
    //     let contribution = p.get_contribution_on(Utc.ymd(2014, 7, 8));
    //     assert_eq!(contribution, 0)
    // }

    // #[test]
    // fn contribution_in_future() {
    //     let name = String::from("My Project");
    //     let start = Utc.ymd(2022, 7, 8);
    //     let duration = Duration::weeks(2);
    //     let approx_value: u32 = 20000;
    //     let p = Project::new(name, approx_value, start, &duration);
    //     let contribution = p.get_contribution_on(Utc.ymd(2024, 7, 8));
    //     assert_eq!(contribution, 0)
    // }
}
