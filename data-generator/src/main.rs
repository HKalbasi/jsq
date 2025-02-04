use clap::Parser;
use rand::{distr::{Alphanumeric, Uniform}, Rng, SeedableRng};
use serde::Serialize;

#[derive(Debug, Parser)]
enum Command {
    PaymentJsonl,
}

#[derive(Serialize)]
struct Payment {
    id: usize,
    name: String,
    amount: i32,
}

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    match Command::parse() {
        Command::PaymentJsonl => {
            for id in 0..10_000_000 {
                let name: String = (&mut rng)
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
                let p = Payment {
                    id,
                    name,
                    amount: rng.sample(Uniform::new(-100_000i32, 100_000).unwrap()),
                };
                println!("{}", serde_json::to_string(&p).unwrap());
            }
        }
    }
}
