extern crate prettytable;

use structopt::StructOpt;

use rpg::{
    generator::{generate_char_password, generate_word_password},
    password::{display, PasswordSummary},
};

/// Generate memorable and secure passwords.
#[derive(StructOpt, Debug)]
#[structopt(name = "Recallable Password Generator")]
struct PasswordGenOpt {
    /// Use characters instead of words
    #[structopt(short = "c", long)]
    use_chars: bool,

    /// Separator to use between words
    #[structopt(short, long, default_value = " ")]
    sep: String,

    /// Length of password. (Character password only)
    #[structopt(short, long, default_value = "28")]
    length: u16,

    /// Number of words to use in password.
    #[structopt(short, long, default_value = "4")]
    words: u8,

    /// Batch size
    #[structopt(short, long, default_value = "5")]
    pub batch: u8,

    /// Remove ending digit from word password.
    #[structopt(short = "d", long)]
    no_ending_digit: bool,
}

impl PasswordGenOpt {
    /// Generates a memorizable word based password or character password.
    pub fn generate_password(&self) -> String {
        if self.use_chars {
            generate_char_password(self.length as usize)
        } else {
            generate_word_password(self.words as usize, &self.sep, !self.no_ending_digit)
        }
    }
}

fn main() {
    let opt = PasswordGenOpt::from_args();

    let passwords: Vec<PasswordSummary> = (0..opt.batch)
        .map(|_| PasswordSummary::new(&opt.generate_password()))
        .collect();

    display(&passwords);
}
