struct Boeing {
    required_crew: u8,
    range: u16
}

struct Airbus {
    required_crew: u8,
    range: u16
}

trait Flight {
    fn is_legal(&self, required_crew: u8, availble_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, availble_crew: u8, range: u16, distance: u16) -> bool {
        if (availble_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, availble_crew: u8, range: u16, distance: u16) -> bool {
        if (availble_crew >= required_crew) && (range + 280 > distance) {
            true
        } else {
            false
        }
    }
}

pub fn side_lesson() {
    let boeing = Boeing {
        required_crew: 4,
        range: 7370
    };

    let airbus = Airbus {
        required_crew: 7,
        range: 5280
    };

    let boeing_is_legal = boeing.is_legal(
        boeing.required_crew, 
        18, 
        boeing.range, 
        2385,
    );

    let airbus_is_legal = airbus.is_legal(
        airbus.required_crew, 
        3, 
        airbus.range, 
        2200
    );

    println!("Is the boeing flight legal? {}\nIs the airbus flight legal? {}", boeing_is_legal, airbus_is_legal);
}