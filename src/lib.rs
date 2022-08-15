//! *hallo* predicts different outcomes of the future
//! by *rolling the dice* many, many times.
//!
//! The main idea is to create a collection of things that
//! span a timeline. Some generate $$$. Some use it.
//!
//! Then we run many scenarios:
//!
//! - what if only the first one happens?
//! - what if the 2nd and 10th one happen?
//!
//! We do that thousands of time to build up a picture of
//! different possible outcomes.
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
    use chrono::{prelude::*, Duration};

    #[test]
    fn it_works() {
        let today = Utc::today();
        let p1 = ProjectBuilder::default()
            .name("p1")
            .duration_weeks(3)
            .start_date(&(today + Duration::weeks(2)))
            .build();

        let p2 = ProjectBuilder::default()
            .name("p2")
            .duration_weeks(5)
            .start_date(&(today + Duration::weeks(8)))
            .build();

        let p3 = ProjectBuilder::default()
            .name("p3")
            .duration_weeks(5)
            .start_date(&(today + Duration::weeks(4)))
            .build();

        println!("{}", p1);
        println!("{}", p2);
        println!("{}", p3);
    }
}
