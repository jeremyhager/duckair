use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_num: f64 = rng.gen();

    println!("{}", random_num);
}

