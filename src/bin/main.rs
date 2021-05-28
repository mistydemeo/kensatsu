use kensatsu::emitter::IOEmitter;
use kensatsu::reader::read_loop;

use std::process::exit;

use pafe::pasori;

fn main() {
    if let Some(pasori) = pasori::Pasori::create() {
        let mut emitter = IOEmitter { target: &mut std::io::stdout() };

        read_loop(pasori, &mut emitter).unwrap();
        exit(0);
    } else {
        eprintln!("No PaSoRi detected!");
        exit(1);
    }
}
