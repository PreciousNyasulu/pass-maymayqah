use encryptfile as ef;
use std::fs;
use crate::utils::utils;


pub struct Encryption{
    passphrase : String
}

impl Encryption{
    pub fn new(_passphrase :&str) -> Self{
        self{
            passphrase : _passphrase.to_string()
        }
    }

    //encrypt password file
    pub fn encrypt_password_file(&self) {
        let mut folder_path = std::env::var("HOME").unwrap();
        folder_path.push_str("/pass_maymayqah");
    
        let file_path = format!("{}/{}", folder_path, "password.json");
        if utils::create_program_folder(&folder_path) {
            utils::create_password_file(&file_path);
        }
    
        let mut c = ef::Config::new();
        c.input_stream(ef::InputStream::File(file_path.to_owned()))
            .output_stream(ef::OutputStream::File("./password_md.ef".to_owned()))
            .add_output_option(ef::OutputOption::AllowOverwrite)
            .initialization_vector(ef::InitializationVector::GenerateFromRng)
            .password(ef::PasswordType::Text(self.passphrase.to_string(), ef::scrypt_defaults()))
            .encrypt();
    
        if let Err(e) = ef::process(&c) {
            panic!("error encrypting: {:?}", e);
        }
    }
      
}
