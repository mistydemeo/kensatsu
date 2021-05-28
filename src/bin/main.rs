use kensatsu::reader::read_loop;

use std::process::exit;

use pafe::pasori;

fn main() {
    if let Some(pasori) = pasori::Pasori::create() {
        read_loop(pasori).unwrap();
        exit(0);
    } else {
        eprintln!("No PaSoRi detected!");
        exit(1);
    }
}
