use clap::{Parser, Subcommand};

mod ch3;
use ch3::TempUnit;

mod ch8;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    TempConverter {
        #[arg(short, long)]
        value: f64,
        #[arg(value_enum)]
        from: TempUnit,
        #[arg(value_enum)]
        to: TempUnit,
    },
    Fibonacci {
        #[arg(short)]
        n: i32,
    },
    Xmas,
    NumAnalysis {
        list: Vec<i32>
    },
    PigLatin {
        word: String
    },
    Console
}

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        Mode::TempConverter { value, from, to } => {
            let temp = ch3::convert_temp(value, from, to);
            println!("{temp}");
        }
        Mode::Fibonacci {n:nth} => {
            println!("{}", ch3::fib(nth));
        }
        Mode::Xmas => {
            ch3::play_song();
        }
        Mode::NumAnalysis { list } => {
            ch8::median_and_mode(list);
        }
        Mode::PigLatin { word } => {
            ch8::pig_latin(word);
        }
        Mode::Console => ch8::console()
    }
}
