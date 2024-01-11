use clipboard::{ClipboardContext,ClipboardProvider};

pub struct Clipboard{
    password : String,
}

pub impl Clipboard{
    fn new(_password: String) -> Self{
        Self{password:_password}
    }
    
    pub fn Copy(&self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(self.password.to_owned()).unwrap();
    }
}