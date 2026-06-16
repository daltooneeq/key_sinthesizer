use eframe::{egui};



fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("sinth", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))));
}

#[derive(Default)]
struct App{
    text: String,

    x: Vec<f64>,
    y: Vec<f64>,
    func_succ: bool,
    clicked: bool,

}
impl App{
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {


            ui.text_edit_singleline(&mut self.text);

            
            if ui.button("New Function").clicked() {
                (self.x, self.y) = get_function(&self.text);
                self.func_succ = check_periodicity(&self.x, &self.y);
                self.clicked = true;
            }
            if !self.func_succ {
                ui.label("Function is not periodicity");
            }
            if self.clicked{

                ui.label(egui::RichText::new(draw_function(&self.x, &self.y))
                    .monospace()
                    .size(2.0)
                );
            }
        });
    }
}

fn get_function (s: &str) -> (Vec<f64>, Vec<f64>) {
    let mut y: Vec<f64> = vec![0.0; 400];
    let mut x: Vec<f64> = vec![0.0; 400];

    for i in 0..400 {
        x[i] = (i as f64)/100.0;

        let copy = s.replace("x", &format!("{}", x[i] * std::f64::consts::PI));


        y[i] = meval::eval_str(&copy).unwrap();

        if y[i] < -1.0 {y[i] = -1.0};
        if y[i] > 1.0 {y[i] = 1.0};

    }
    (x, y)

}
fn check_periodicity(x: &Vec<f64>, y: &Vec<f64>) -> bool{
    let normal_error = 1e-3; 
    for i in 0..200 {
        if (y[i] - y[i+200]).abs() > normal_error {return false}
    }
    true
}
fn draw_function(x: &Vec<f64>, y: &Vec<f64>) -> String{


    let mut res = vec![vec![' '; 400]; 21];

    for i in (0..400).step_by(1) {
        res[(((y[i]+1.0)*10.0).round()) as usize][i/1] = '∎';
    }


    let mut result_str = String::new();
    for i in (0..=20).rev() {
        for el in &res[i]{
            result_str = result_str + &el.to_string();
        }
        result_str += "\n";
    }

    result_str


}