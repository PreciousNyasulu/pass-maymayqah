// src/main.rs

use clap::{App, Arg};
use rand::Rng;

struct Password {
    length: usize,
}

impl Password {
    fn new(len: usize) -> Self {
        Self { 
            length: len 
        }
    }

    fn generate(&self) {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| characters.chars().nth(rng.gen_range(0..characters.len())).unwrap())
            .collect();
        let mut comment = "";
        if self.length < 8 {
            comment = "//For security reasons, we recommend 8-plus character length"
        }
        println!("Here is your password: {}  {}", password,comment)
    }
}

fn main() {
    let matches = App::new("pass-maymayqah")
        .version("1.0")
        .author("Your Name")
        .about("A basic CLI app for generating passwords")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("LENGTH")
            .help("Sets the password's length")
            .required(true)
            .takes_value(true))
        .get_matches();

    let len: usize = matches.value_of("length").unwrap_or_default().parse().unwrap_or_default();

    let cli_app = Password::new(len);

    cli_app.generate();
}