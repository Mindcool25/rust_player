use rodio::{OutputStream, Sink};
//use std::path::PathBuf;

pub mod player_ui;
pub mod playback;

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let native_options = eframe::NativeOptions::default();
    let music_ui = Box::new(player_ui::MusicPlayer {
        playlist: Vec::new(),
        sink,
        volume: 50.0,
    });
    let _ = eframe::run_native(
        "Rust Player",
        native_options,
        Box::new(|_cc| {
            music_ui
        }),
    );
}

