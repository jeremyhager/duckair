fn main() {
    let availble_aircraft = "Boeing";
    let min_crew = 7;
    let avail_crew = 4;

    if (availble_aircraft == "Boeing" 
        || availble_aircraft == "Airbus")
        && min_crew <= avail_crew {
        println!("OK!");
    }
}
