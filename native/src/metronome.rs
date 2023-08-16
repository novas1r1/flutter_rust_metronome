use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct Metronome {
    bpm: u32,
    is_playing: Arc<AtomicBool>,
}

impl Metronome {
    pub fn new() -> Self {
        Metronome {
            bpm: 60, // default bpm
            is_playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm;
    }

    pub fn play(&mut self) {
        self.is_playing.store(true, Ordering::Relaxed);
        let interval = Duration::from_millis(60000 / self.bpm as u64);

        let is_playing_clone = self.is_playing.clone();
        thread::spawn(move || {
            while is_playing_clone.load(Ordering::Relaxed) {
                log::info!("Tick");
                thread::sleep(interval);
            }
        });
    }

    pub fn stop(&mut self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }
}

