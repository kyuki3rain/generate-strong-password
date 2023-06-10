use clap::Parser;
use rand::{seq::SliceRandom, Rng};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Includes uppercase letters in the password
    #[arg(short = 'C', long)]
    uppercase: bool,

    /// Includes lowercase letters in the password
    #[arg(short = 'c', long)]
    lowercase: bool,

    /// Includes numbers in the password
    #[arg(short, long)]
    numbers: bool,

    /// Sets the symbols to include in the password
    #[arg(short, long, default_value_t = String::from(""))]
    symbols: String,
}

fn main() {
    let args = Args::parse();

    let password_length = args.length;

    let include_uppercase = args.uppercase;
    let include_lowercase = args.lowercase;
    let include_numbers = args.numbers;
    let symbol_sets: Vec<char> = args.symbols.chars().collect();
    let include_symbols = !symbol_sets.is_empty();

    if !include_uppercase && !include_lowercase && !include_numbers && !include_symbols {
        eprintln!("Please specify at least one type of character to include");
        process::exit(1);
    }

    let character_sets: Vec<Vec<char>> = vec![
        vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ],
        vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ],
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        symbol_sets,
    ];

    let mut available_character_sets: Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut password = String::new();

    if include_uppercase {
        available_character_sets.push(0);
        password.push(*character_sets[0].choose(&mut rng).unwrap());
    }
    if include_lowercase {
        available_character_sets.push(1);
        password.push(*character_sets[1].choose(&mut rng).unwrap());
    }
    if include_numbers {
        available_character_sets.push(2);
        password.push(*character_sets[2].choose(&mut rng).unwrap());
    }
    if include_symbols {
        available_character_sets.push(3);
        password.push(*character_sets[3].choose(&mut rng).unwrap());
    }

    for _ in 0..password_length - available_character_sets.len() {
        let set = &character_sets[*available_character_sets.choose(&mut rng).unwrap()];
        password.push(*set.choose(&mut rng).unwrap());
    }

    println!("↓↓↓ Generated password ↓↓↓\n{}", password);
}
