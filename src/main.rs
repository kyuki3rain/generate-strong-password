use clap::Parser;
use rand::seq::SliceRandom;
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

    let mut symbol_sets: Vec<char> = args.symbols.chars().collect();
    let include_symbols = !symbol_sets.is_empty();

    if !include_uppercase && !include_lowercase && !include_numbers && !include_symbols {
        eprintln!("Please specify at least one type of character to include");
        process::exit(1);
    }

    let mut rng = rand::thread_rng();
    let mut password = Vec::new();

    let mut character_sets: Vec<char> = Vec::new();
    let mut offset = 0;

    if include_uppercase {
        let mut uppercase_sets = ('A'..='Z').collect::<Vec<char>>();
        password.push(*uppercase_sets.choose(&mut rng).unwrap());
        offset += 1;

        character_sets.append(&mut uppercase_sets);
    }
    if include_lowercase {
        let mut lowercase_sets = ('a'..='z').collect::<Vec<char>>();
        password.push(*lowercase_sets.choose(&mut rng).unwrap());
        offset += 1;

        character_sets.append(&mut lowercase_sets);
    }
    if include_numbers {
        let mut number_sets = ('0'..='9').collect::<Vec<char>>();
        password.push(*number_sets.choose(&mut rng).unwrap());
        offset += 1;

        // 数字は3倍出やすいようにする
        character_sets.append(&mut number_sets);
        character_sets.append(&mut number_sets);
        character_sets.append(&mut number_sets);
    }
    if include_symbols {
        password.push(*symbol_sets.choose(&mut rng).unwrap());
        offset += 1;

        character_sets.append(&mut symbol_sets);
    }

    if password_length < offset {
        eprintln!(
            "Password length must be greater than or equal to {}",
            offset
        );
        process::exit(1);
    }

    for _ in 0..password_length - offset {
        password.push(*character_sets.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);

    let password: String = password.into_iter().collect();

    println!("↓↓↓ Generated password ↓↓↓\n{}", password);
}
