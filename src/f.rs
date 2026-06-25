pub fn get_function (s: &str) -> (Vec<f32>, Vec<f32>) {
    let mut y: Vec<f32> = vec![0.0; 400];
    let mut x: Vec<f32> = vec![0.0; 400];

    for i in 0..400 {
        x[i] = (i as f32)/100.0;

        let copy = s.replace("x", &format!("{}", x[i] * std::f32::consts::PI));


        y[i] = (meval::eval_str(&copy).unwrap() as f32);

        if y[i] < -1.0 {y[i] = -1.0};
        if y[i] > 1.0 {y[i] = 1.0};

    }
    (x, y)

}
pub fn check_periodicity(x: &Vec<f32>, y: &Vec<f32>) -> bool{
    let normal_error = 1e-3; 
    for i in 0..200 {
        if (y[i] - y[i+200]).abs() > normal_error {return false}
    }
    true
}
pub fn draw_function(x: &Vec<f32>, y: &Vec<f32>) -> String{


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