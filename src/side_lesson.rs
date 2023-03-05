#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U
}

pub fn side_lesson() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is a VOR")
    };

    let ndb_data:Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data
    };

    println!("VOR info is: {:?}", vor);
    println!("NDB info is: {:?}", ndb);
}