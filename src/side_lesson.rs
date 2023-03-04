use std::collections::HashMap;

pub fn side_lesson() {
    let mut flights = HashMap::new();

    flights.insert("DA918", ("11:12", "Orlando"));
    flights.insert("DA428", ("12:05", "Salt Lake City"));
    flights.insert("DA98", ("09:43", "London"));
    flights.insert("DA113", ("06:20", "Boston"));
    flights.insert("DA41", ("15:30", "Berlin"));
    flights.insert("DA2815", ("17:11", "Nashville"));

    let flight_number = "DA531";
    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("22:00", "Puerto Rico"));
    } else {
        println!("Flight {} is already entered", flight_number)
    }

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}