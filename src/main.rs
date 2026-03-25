use eframe::egui;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, MixerDeviceSink, source::Source};
use walkdir::WalkDir;

struct MusicPlayer {
    player: rodio::Player,
    _handle: rodio::MixerDeviceSink,
    volume: f32,
    speed: f32,
    songs: Vec<String>,
}

impl MusicPlayer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default");
        let player = rodio::Player::connect_new(&handle.mixer());

        let mut found_songs = Vec::new();
        for entry in WalkDir::new("./music").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path().display().to_string();
                found_songs.push(path);
            }
        }

        Self {
            player,
            _handle: handle,
            volume: 0.5,
            speed: 1.0,
            songs: found_songs,
        }
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Spike - Music Player");

            ui.add_space(10.0);


            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Volume").vertical());
                self.player.set_volume(self.volume);

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.add(egui::Slider::new(&mut self.speed, 0.0..=2.0).text("Speed").vertical());
                self.player.set_speed(self.speed);

            });


            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Selcet song: ");
                for path in &self.songs {
                    if ui.button(path).clicked() {
                        let file = File::open(path).expect("not found");
                        let reader = BufReader::new(file);
                        let source = Decoder::new(reader).expect("error");
                        self.player.append(source);

                    }

                }
            });


            ui.horizontal(|ui| {
                if ui.button(">").clicked(){
                    self.player.play();


                }
                if ui.button("=").clicked(){
                    self.player.pause();
                }

            })
        });
    }
}
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Music Player",
        native_options,
        Box::new(|cc| Ok(Box::new(MusicPlayer::new(cc))))
    )
}
