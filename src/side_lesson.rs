pub fn side_lesson() {
    let mut original = String::from("original value");
    println!("\nOuter original value: \t\"{}\"", original);
    
    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner original next: \t\"{}\"", next);
        println!("inner original original: \t\"{}\"", original);
    }

    original = String::from("new value");
    println!("\nOuter original value: \t\"{}\"", original);
}