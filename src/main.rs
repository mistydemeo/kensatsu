use std::collections::HashMap;
use std::process::exit;

use pafe::pasori;

fn read_loop(pasori: pasori::Pasori) -> Result<(), String> {
    let mut hm = HashMap::new();
    loop {
        if let Some(felica_tag) = pasori.poll(pasori::CardType::Any, pasori::Timeslot::N0) {
            let idm = felica_tag.tag.IDm;
            if !hm.contains_key(&idm) {
                hm.insert(idm, true);
                println!("Scanned card {:02x?}", idm);
            }
        }
    }
}

fn main() {
    let pasori = pasori::Pasori::create();
    match pasori.init() {
        Ok(_) => {},
        Err(_) => {
            eprintln!("No PaSoRi detected!");
            exit(1);
        },
    }
    read_loop(pasori).unwrap();

    exit(0);
}
