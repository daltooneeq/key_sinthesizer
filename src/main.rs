mod key;
mod f;
mod sound;

use eframe::{egui};

use cpal::{Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

use std::thread;


fn main() {
    thread::spawn(|| {
        key::input_listen();
    });


    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap();
    println!("cfg: {:?}", config);
    let sample_rate = config.sample_rate() as f32;
    let channels = config.channels() as usize;


    let native_options = eframe::NativeOptions::default();
    eframe::run_native("sinth", native_options, Box::new(|cc| Ok(Box::new(App::new(cc, device, config)))));
}

struct App{
    text: String,

    x: Vec<f32>,
    y: Vec<f32>,
    func_succ: bool,
    clicked: bool,

    device: cpal::Device,
    config: cpal::SupportedStreamConfig,

    stream: Option<cpal::Stream>,

}
impl App{
    fn new(cc: &eframe::CreationContext<'_>, device: cpal::Device, config: cpal::SupportedStreamConfig) -> Self {
        Self {
            text: "".to_string(),
            x: vec![],
            y: vec![],
            func_succ: false,
            clicked: false,
            device: device,
            config: config,
            stream: None,
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {


            ui.text_edit_singleline(&mut self.text);

            
            if ui.button("New Function").clicked() {
                (self.x, self.y) = f::get_function(&self.text);
                self.func_succ = f::check_periodicity(&self.x, &self.y);
                self.clicked = true;
            }
            if ui.button("Test").clicked() {
                let stream = sound::make(&self.device, &self.config, 220.0, 48000.0, &self.y);
                stream.play().unwrap();
                self.stream = Some(stream);
            }
            if !self.func_succ {
                ui.label("Function is not periodicity");
            }
            if self.clicked{

                ui.label(egui::RichText::new(f::draw_function(&self.x, &self.y))
                    .monospace()
                    .size(2.0)
                );
            }
        });
    }
}

