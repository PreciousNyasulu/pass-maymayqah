# pass-maymayqah

A simple command-line interface (CLI) application for generating secure passwords in Rust 🦀.

**Side-Note**: The word `maymayqah`means generator in klingon(star-trek 😁).

## Description

This Rust-based password generator provides a straightforward way to generate strong and secure passwords. It allows you to customize the length of the password and provides optional comments on password strength based on the specified length.

## Usage
### Command-line Options

- `-l`, `--length`: Sets the password length.

### Examples

Generate a password with a length of 16 characters:
```bash
$ pass-maymayqah -l 12    //12 being the number of password characters
```

### Prerequisites

- Rust (including Cargo) installed on your system. If not, you can install Rust using [Rustup](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/preciousnyasulu/pass-maymayqah.git
   cd pass-maymayqah
   ```

2. Build the program using Cargo:

   ```bash
   cargo build --release
   ```


### Notes on Password Strength

If the specified password length is less than 8 characters, a comment will be provided, suggesting a longer length for security reasons.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributors
@PreciousNyasulu