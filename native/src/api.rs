use crate::metronome::Metronome;

pub fn create() -> Metronome {
    log::info!("Metronome is created");
    Metronome::new()
}

pub fn set_bpm(metronome: &mut Metronome, bpm: u32) {
    log::info!("Metronome is set to {} bpm", bpm);
    // let mut metronome = Metronome::new();
    metronome.set_bpm(bpm);
}

pub fn play(metronome: &mut Metronome) {
    log::info!("Metronome is playing");
    // let mut metronome = Metronome::new();
    metronome.play();
}

pub fn stop(metronome: &mut Metronome) {
    log::info!("Metronome is stopped");
    // let mut metronome = Metronome::new();
    metronome.stop();
} 

pub fn init_logger() {
    log::info!("This will be visible in Logcat with the tag 'Rust'");
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));
} 


