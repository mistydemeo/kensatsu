#[cfg(target_os = "linux")]
use crate::evdev::Device;

pub trait Emitter {
    fn emit(&mut self, value: i32) -> Result<(), std::io::Error>;
}

pub struct IOEmitter<'a> {
    pub target: &'a mut dyn std::io::Write,
}

impl Emitter for IOEmitter<'_> {
    fn emit(&mut self, value: i32) -> Result<(), std::io::Error> {
        let message = format!("Emitting value: {}\n", value);
        self.target.write_all(message.as_bytes())?;

        Ok(())
    }
}

#[cfg(target_os = "linux")]
pub struct EVDevEmitter {
    pub device: Device,
}
#[cfg(target_os = "linux")]
impl Emitter for EVDevEmitter {
    fn emit(&mut self, value: i32) -> Result<(), std::io::Error> {
        self.device.write(value)?;

        Ok(())
    }
}
