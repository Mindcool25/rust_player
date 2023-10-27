use rodio::Sink;
use eframe::egui;
use std::path::PathBuf;
use crate::playback::get_song;

pub struct Song {
    pub path: PathBuf,
    pub index: i32,
}

pub struct MusicPlayer {
    pub playlist: Vec<PathBuf>,
    pub sink: Sink,
    pub volume: f32,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        // Setting up a default sink
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        sink.set_volume(0.50); // Change this to whatever was last
        Self {
            playlist: Vec::new(),
            sink,
            volume: 50.0,
        }
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sink management
        self.sink.set_volume(self.volume/100.0);
        if self.sink.empty() && !self.playlist.is_empty() {
            self.playlist.rotate_left(1);
            self.sink.append(get_song(self.playlist[0].clone()));
        }

        // UI definition
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Player");
            // Skip button
            if ui.button("Skip").clicked() {
                self.skip();
            }
            // Play/Pause
            if ui.button("Play/Pause").clicked() {
                self.toggle();
            }
            // Volume slider/label
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).text("VOL"));
            ui.heading(format!("Volume: {}", self.sink.volume()*100.0));

            // Drag/drop file handling
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    for file in &i.raw.dropped_files {
                        self.playlist.push(file.path.clone().unwrap())
                    }
                    println!("File Paths: {:?}", self.playlist);
                }
            });
        });
    }
}

impl MusicPlayer {
    fn skip(&self) {
        self.sink.skip_one();
        println!("Skipped");
    }
    fn toggle(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        }
        else {
            self.sink.pause();
        }
    }
}
