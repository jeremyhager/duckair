fn main() {
    let ndb_freq:u16 = 384;

    match ndb_freq {
        200..=500 => {
            println!("NDB frequency is valid");
        }

        _ => {
            println!("NDB frequency is NOT valid");
        }
    }
}
