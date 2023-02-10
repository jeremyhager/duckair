fn main() {
    let squared = i32::pow(8, 2);
    let floating_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("integer: {}", squared);
    println!("float to int: {}", floating_integer);
    println!("float to float: {}", float_float);
}
