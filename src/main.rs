use chrono::{Duration, Utc};
use hallo::projects::ProjectBuilder;

pub fn main() {
    let today = Utc::today();
    let p1 = ProjectBuilder::default()
        .name("p1")
        .duration_weeks(3)
        .start_date(&(today + Duration::weeks(2)))
        .build();

    let p2 = ProjectBuilder::default()
        .name("p2")
        .duration_weeks(5)
        .value(5000)
        .start_date(&(today + Duration::weeks(8)))
        .build();

    let p3 = ProjectBuilder::default()
        .name("p3")
        .duration_weeks(5)
        .value(1000)
        .start_date(&(today + Duration::weeks(4)))
        .build();

    println!("{}", p1);
    println!("{}", p2);
    println!("{}", p3);
}
