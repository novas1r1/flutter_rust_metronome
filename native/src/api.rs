use crate::metronome::Metronome;

pub fn play(bpm: u32) {
    log::info!("Metronome is playing");
    let mut metronome = Metronome::new();
    metronome.play(bpm);
}

pub fn stop() {
    log::info!("Metronome is stopped");
    let mut metronome = Metronome::new();
    metronome.stop();
} 

pub fn init_logger() {
    log::info!("This will be visible in Logcat with the tag 'Rust'");
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));
} 


//  pub struct Api {
//     metronome: Metronome,
// }

// impl  Api {
//     pub fn new() -> Self {
//         Api {
//             metronome: Metronome::new(),
//         }
//     }    
    
//     pub fn play(&mut self, bpm: u32) {
//         log::info!("Metronome is playing");
//         self.metronome.play(bpm);
//     }

//     pub fn stop(&mut self) {
//         log::info!("Metronome is stopped");
//         self.metronome.stop();
//     }
// }

// static METRONOME: Lazy<Metronome> = Lazy::new(Metronome::new);



