use rand::Rng;
use colored::Colorize;
use crate::clipboard::Clipboard;

pub struct Password {
    length: usize,
    copy: bool
}

impl Password {
    pub fn new(_length: usize,_copy : bool) -> Self {
        Self { 
            length: _length, 
            copy : _copy,
        }
    }

    pub fn generate(&self) {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| characters.chars().nth(rng.gen_range(0..characters.len())).unwrap())
            .collect();

        let mut comment = String::new();
        if self.length < 8 {
            comment = "// For security reasons, we recommend 8-plus character length".yellow().to_string();
        }
        
        if self.copy {
            let clipboard_events : Clipboard = Clipboard::new(password.to_owned());
            clipboard_events.copy();
        }
        
        println!("Here is your password: {}  {}", password.green(), comment.red())
    }
}