// src/main.rs

use clap::{App, Arg};
use pass_maymayqah::password::Password;

fn main() {
    let matches = App::new("pass-maymayqah")
        .version("1.0")
        .author("Precious Nyasulu")
        .about("A basic CLI app for generating passwords")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("LENGTH")
            .help("Sets the password's length")
            .required(true)
            .takes_value(true))
        .arg(
            Arg::with_name("copy")
            .short("cp")
            .long("copy")
            .help("Copies the password to clipboard")
        )
        .get_matches();

    let length: usize = matches.value_of("length").unwrap_or_default().parse().unwrap_or_default();

    // copies the password to clipboard if flag is present
    let copy : bool = matches.is_present("copy");

    let cli_app: Password = Password::new(length,copy);

    cli_app.generate();
}
