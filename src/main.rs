use crate::password_generator::PasswordGenerator;
use clap::Parser;

mod character_set;
mod password_generator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the length of the password
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Sets the weights for uppercase letters in the password
    #[arg(short = 'C', long, default_value_t = 4)]
    uppercase: usize,

    /// Sets the weights for lowercase letters in the password
    #[arg(short = 'c', long, default_value_t = 4)]
    lowercase: usize,

    /// Sets the weights for numbers in the password
    #[arg(short, long, default_value_t = 4)]
    numbers: usize,

    /// Sets the weights for symbols in the password
    #[arg(short, long, default_value_t = 1)]
    symbols: usize,

    /// Sets the symbols to include in the password
    #[arg(long, default_value_t = String::from("!$%&'()*+,/;<=>?[]^{}~"))]
    symbol_sets: String,
}

fn main() {
    let args = Args::parse();

    let password_length = args.length;

    let weights = vec![args.uppercase, args.lowercase, args.numbers, args.symbols];

    if weights.iter().all(|&x| x == 0) {
        eprintln!("Error: Please specify at least one type of character to include");
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();
    let generator = PasswordGenerator::new(password_length, weights, &args.symbol_sets);

    match generator.generate(&mut rng) {
        Ok(password) => println!("{}", password),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
