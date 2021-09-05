#[cfg(target_os = "linux")]
use kensatsu::emitter::EVDevEmitter;
use kensatsu::emitter::{Emitter, IOEmitter};
#[cfg(target_os = "linux")]
use kensatsu::evdev::Device;
use kensatsu::reader::read_loop;

use std::io::{Error, ErrorKind};
use std::process::exit;

use kensatsu::config::Config;
use pafe::pasori;

fn run(emitter: &mut dyn Emitter) -> Result<(), confy::ConfyError> {
    if let Some(pasori) = pasori::Pasori::create() {
        read_loop(pasori, emitter).unwrap();
        exit(0);
    } else {
        eprintln!("No PaSoRi detected!");
        exit(1);
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let config: Config = confy::load("kensatsu", None)?;

    // Define this here, even if it ends up unused, because
    // defining it within the match statement would allow it to
    // fall out of scope before being used.
    let mut stdout = std::io::stdout();
    match config.emitter.as_str() {
        "io" => run(&mut IOEmitter {
            target: &mut stdout,
        }),
        // Only valid on Linux.
        #[cfg(not(target_os = "linux"))]
        "evdev" => {
            let e = Error::new(ErrorKind::Other, "evdev only valid on Linux");
            Err(confy::ConfyError::GeneralLoadError(e))
        }
        #[cfg(target_os = "linux")]
        "evdev" => run(&mut EVDevEmitter {
            device: Device::create().unwrap(),
        }),
        _ => {
            let e = Error::new(
                ErrorKind::Other,
                "supported emitter values are `io` or `evdev`",
            );
            Err(confy::ConfyError::GeneralLoadError(e))
        }
    }
}
