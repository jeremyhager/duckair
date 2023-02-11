fn main() {

    let have_driver_licese = false;
    let have_passport = false;
    let have_proof = have_driver_licese || have_passport;

    let have_boarding_pass = true;
    let have_id = have_proof;
    let can_board  = have_boarding_pass && have_id;

    println!("Boarding Pass: {}, ID: {}", have_boarding_pass, have_id);
    println!("Can board plane: {}", can_board);
}
