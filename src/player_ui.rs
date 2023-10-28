use audiotags::Tag;
use rodio::Sink;
use eframe::egui;
use std::path::PathBuf;
use std::path::Path;
use crate::playback::get_song;

#[derive(Debug, Clone)]
pub struct Song {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
}

pub struct MusicPlayer {
    pub playlist: Vec<Song>,
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
            // Might change how this works later
            self.playlist.rotate_right(1);
            self.sink.append(get_song(self.playlist[0].path.clone()));
        }

        // UI definition
        // TODO: Write playlist view, try out panels to make it look nice.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Player");
            // Skip button
            if ui.button("next").clicked() {
                self.next();
            }
            if ui.button("last").clicked() {
                self.last();
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
                        if !Path::new(&file.path.clone().unwrap()).is_dir() {
                            self.playlist.push(self.parse_song(file.path.clone().unwrap()));
                        }
                        else {
                            println!("That is a dir :(");
                        }
                    }
                    println!("File Paths: {:?}", self.playlist);
                }
            });
        });
    }
}

impl MusicPlayer {
    fn next(&mut self) {
        self.playlist.rotate_right(1);
        self.sink.skip_one();
        self.sink.append(get_song(self.playlist[0].path.clone()));
    }
    fn last(&mut self) {
        self.playlist.rotate_left(1);
        self.sink.skip_one();
        self.sink.append(get_song(self.playlist[0].path.clone()));
    }
    fn toggle(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        }
        else {
            self.sink.pause();
        }
    }

    fn parse_song(& self, filename: PathBuf) -> Song {
        let tag = Tag::new().read_from_path(filename.clone()).unwrap();
        let song_title = tag.title().unwrap_or("").to_string();
        let song_artist = tag.artist().unwrap_or("").to_string();
        Song {
            title: song_title,
            artist: song_artist,
            path:filename
        }
    }
}
