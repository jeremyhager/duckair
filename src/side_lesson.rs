use std::ops::Add;

pub fn side_lesson() {
    let sum = add(256, 262);
    println!("{}", sum);
}

fn add<T>(operand1: T, operand2: T) -> T
where T: Add<Output =T>
{
    operand1 + operand2
}