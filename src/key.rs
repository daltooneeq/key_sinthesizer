use std::string::FromUtf8Error;

use rdev::{listen, Event};

pub fn input_listen() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
    fn callback(event: Event) {
        println!("callback {:?}", event.event_type);
        if let Some(string) = event.name {
            println!("{:?}", string)
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct Note {
    name: String,
    octave: u8,
    frequency: f64,
}

pub fn get_notes(freq: f64) -> Vec<Note>{
    let mut notes = vec![Note::default(); 84];
    for i in 0..7 {
        let names = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

        for j in 0..12 {
            notes[i*12+j].name = names[j].to_string();
            notes[i*12+j].octave = (i as u8);
        }
    };
    notes[33].frequency = freq;
    for i in 34..84 {
        notes[i].frequency = notes[i-1].frequency * (2.0 as f64).powf(1.0/12.0);
    }
    for i in (0..33).rev() {
        notes[i].frequency = notes[i+1].frequency / (2.0 as f64).powf(1.0/12.0);
    }
    notes
}