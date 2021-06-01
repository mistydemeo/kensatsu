#[cfg(not(target_os = "linux"))]
use kensatsu::emitter::IOEmitter;
#[cfg(target_os = "linux")]
use kensatsu::evdev::Device;
#[cfg(target_os = "linux")]
use kensatsu::emitter::EVDevEmitter;
use kensatsu::reader::read_loop;

use std::process::exit;

use pafe::pasori;

fn main() {
    if let Some(pasori) = pasori::Pasori::create() {
        // TODO: make the IOEmitter an option on Linux
        #[cfg(target_os = "linux")]
        let mut emitter = EVDevEmitter { device: Device::create().unwrap() };
        #[cfg(not(target_os = "linux"))]
        let mut emitter = IOEmitter { target: &mut std::io::stdout() };

        read_loop(pasori, &mut emitter).unwrap();
        exit(0);
    } else {
        eprintln!("No PaSoRi detected!");
        exit(1);
    }
}
