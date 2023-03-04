use std::collections::{VecDeque};

pub fn side_lesson() {
    let mut flights:VecDeque<&str> = VecDeque::new();

    flights.push_front("Orlando departs at 11:12");
    flights.push_back("SLC departs at 12:05");
    flights.push_front("London departs at 09:43");
    flights.push_front("Boston departs at 06:20");
    flights.push_back("Berlin departs at 15:30");
    flights.push_back("Nashville departs at 17:11");

    for flight in flights.iter() {
        println!("{}", flight);
    }

    let length = flights.len();
    println!("There are {} flights", length);
}