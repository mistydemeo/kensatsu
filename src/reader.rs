use crate::emitter::Emitter;

use std::collections::HashMap;
use std::time::Instant;

use pafe::pasori;

fn should_back_off(timestamp: Instant) -> bool {
    Instant::now().duration_since(timestamp).as_secs() < 1
}

pub fn read_loop(pasori: pasori::Pasori, emitter: &mut dyn Emitter) -> Result<(), std::io::Error> {
    let mut hm = HashMap::new();
    // Used to track when a card is lifted
    // Note that all zeroes is not a valid IDm, so we can safely
    // use that as a "null" value.
    let mut last_read = [0; 8];
    loop {
        if let Some(felica_tag) = pasori.poll(pasori::CardType::Any, pasori::Timeslot::N0) {
            let idm = felica_tag.tag.IDm;
            // If the last card scanned was this card, do nothing.
            // `last_read` should be the null IDm if the card was lifted.
            // Protects against repeatedly acting on the same card while
            // it's sitting on the reader.
            if last_read == idm {
                continue;
            }

            // Decide if enough time has passed since the last read
            if let Some(backoff_timetamp) = hm.get(&idm) {
                if should_back_off(*backoff_timetamp) {
                    continue;
                }
            }
            println!("Scanned card {:02x?}", idm);
            emitter.emit(1)?;

            last_read = idm;
        } else {
            hm.insert(last_read, Instant::now());
            last_read = [0; 8];
        }
    }
}
