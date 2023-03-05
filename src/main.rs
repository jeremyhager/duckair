use rand::Rng;
use side_lesson::side_lesson;
mod side_lesson;

fn main() {
    let mut rng = rand::thread_rng();
    let random_num: f64 = rng.gen();
    println!("{}", random_num);

    side_lesson();
}

