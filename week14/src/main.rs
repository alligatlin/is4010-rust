mod generator;
mod validator;

use clap::{Parser, Subcommand};
use generator::{generate_passphrase, generate_pin, generate_random};
use validator::{calculate_entropy, check_common_patterns, validate_strength};

#[derive(Parser)]
#[command(name = "passgen", version, about = "Generate and validate passwords")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Random {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        #[arg(short, long)]
        symbols: bool,
    },
    Passphrase {
        #[arg(short, long, default_value_t = 4)]
        words: usize,
        #[arg(short, long, default_value_t = '-')]
        separator: char,
    },
    Pin {
        #[arg(short, long, default_value_t = 6)]
        length: usize,
    },
    Validate {
        password: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            let pwd = generate_random(length, symbols);
            println!("Generated Random Password: {}", pwd);
            println!("Entropy: {:.2} bits", calculate_entropy(&pwd));
        }
        Commands::Passphrase { words, separator } => {
            let phrase = generate_passphrase(words, separator);
            println!("Generated Passphrase: {}", phrase);
        }
        Commands::Pin { length } => {
            let pin = generate_pin(length);
            println!("Generated PIN: {}", pin);
        }
        Commands::Validate { password } => {
            let strength = validate_strength(&password);
            let common = check_common_patterns(&password);
            println!("Strength: {}", strength);
            if common {
                println!("⚠️ WARNING: This password matches a common weak pattern!");
            }
        }
    }
}
