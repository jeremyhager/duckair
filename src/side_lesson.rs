pub fn side_lesson() {
    let name = "Duck Airlines";

    let write_message = |slogan: String| -> String {
        String::from(format!("{}. {}", name, slogan))
    };

    let phrase = write_message(String::from("We hit the ground every time."));

    println!("{}", phrase);

    // let slogan = "We hit the ground every time.";
    // println!("Welcome to {}. {}", name, slogan)
}