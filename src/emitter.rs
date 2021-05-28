pub trait Emitter {
    fn emit(&mut self, value: u32) -> Result<(), std::io::Error>;
}

pub struct IOEmitter<'a> {
    pub target: &'a mut dyn std::io::Write,
}

impl Emitter for IOEmitter<'_> {
    fn emit(&mut self, value: u32) -> Result<(), std::io::Error> {
        let message = format!("Emitting value: {}\n", value);
        self.target.write(&message.as_bytes())?;

        Ok(())
    }
}
