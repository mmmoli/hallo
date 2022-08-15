//! *hallo* predicts different outcomes of the future by *rolling the dice* many, many times.
//!
//! # Features
//!
//! - Plan *Projects*, *Expertise* and *Costs*
//!
//! # Usage
//!
//! Coming soon

pub mod allocation;
pub mod projects;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::projects::ProjectBuilder;
    use chrono::prelude::*;

    #[test]
    fn it_works() {
        let today = Utc::today();
        let p1 = ProjectBuilder::default()
            .name("p1")
            .duration_weeks(3)
            .build();
        let p2 = ProjectBuilder::default()
            .name("p2")
            .duration_weeks(5)
            .build();
        let p3 = ProjectBuilder::default()
            .name("p3")
            .duration_weeks(5)
            .build();
    }
}
