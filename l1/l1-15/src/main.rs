use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut vals: Vec<u64> = (0..100).map(|_| rng.gen_range(20)).collect();

    println!("{:?}", vals);
}