pub fn side_lesson() {
    let referenced_int = 6;
    let returned_value = return_one_parameter(&referenced_int);
    println!("{}", returned_value);
}

fn return_one_parameter(value: &i32) -> &i32{
    value
}