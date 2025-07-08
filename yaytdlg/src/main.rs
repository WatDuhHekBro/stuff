use eframe::egui;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};

fn main() {
    fs::remove_file("test.mp3").ok();

    /*eframe::run_native(
        &format!(
            "yaytdlg - Yet Another youtube-dl GUI (v{})",
            env!("CARGO_PKG_VERSION")
        ),
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(App::default())),
    )
    .unwrap();*/

    /*let program = "youtube-dl";
    let args = [
        "https://www.youtube.com/watch?v=EAaInaeZfCw",
        "-f",
        "bestvideo+bestaudio/best",
        "--abort-on-unavailable-fragment",
        "--audio-quality",
        "0",
    ];*/
    let program = "rclone";
    let args = [
        "sync",
        "/home/watduhhekbro/tmp/yeet",
        "/home/watduhhekbro/tmp/delet",
    ];
    let mut asdf = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut asdf = asdf.stdout.take().unwrap();

    /*let mut yeet: [u8; 100] = [0; 100];
    let zxcv = asdf.read(&mut yeet);
    println!("{zxcv:?}");*/

    let mut bufread = BufReader::new(asdf);
    let mut buf = String::new();

    while let Ok(n) = bufread.read_line(&mut buf) {
        if n > 0 {
            println!("Line: {}", buf.trim());
            buf.clear();
        } else {
            break;
        }
    }

    /*let reader = BufReader::new(asdf);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("[LINE] {line}"));*/
}

enum Mode {
    Default,
    Processing,
    Done,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Default
    }
}

#[derive(Default)]
struct App {
    mode: Mode,
    console: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                Mode::Default => {
                    ui.heading("Begin the Yeeting");

                    if ui.button("Submit").clicked() {
                        self.mode = Mode::Processing;
                    }
                }
                Mode::Processing => {
                    ui.add_enabled_ui(false, |ui| {
                        ui.text_edit_multiline(&mut self.console);
                    });

                    if ui.button("tmp").clicked() {
                        self.mode = Mode::Done;
                    }
                }
                Mode::Done => {
                    ui.label("Your batch has finished processing.");
                }
            };
        });
    }
}
