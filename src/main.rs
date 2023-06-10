use crate::password_generator::PasswordGenerator;
use clap::Parser;

mod password_generator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Includes uppercase letters in the password
    #[arg(short = 'C', long, default_value_t = 4)]
    uppercase: usize,

    /// Includes lowercase letters in the password
    #[arg(short = 'c', long, default_value_t = 4)]
    lowercase: usize,

    /// Includes numbers in the password
    #[arg(short, long, default_value_t = 4)]
    numbers: usize,

    /// Sets the symbols to include in the password
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
        panic!("Please specify at least one type of character to include");
    }

    let mut rng = rand::thread_rng();
    let generator = PasswordGenerator::new(password_length, weights, args.symbol_sets);

    println!("↓↓↓ Generated password ↓↓↓\n{}", generator.gen(&mut rng));
}
