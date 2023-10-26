use std::io::prelude::*;

pub trait WriteMessage {
    fn write_message(&mut self, message: &str) -> std::io::Result<()>;
}

impl<T: Write> WriteMessage for T {
    fn write_message(&mut self, message: &str) -> std::io::Result<()> {
        self.write_all(message.as_bytes())
    }
}
