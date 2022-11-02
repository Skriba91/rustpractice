use rand::Rng;

pub fn run() {
    const PI: f64 = std::f64::consts::PI;
    let mut rng = rand::thread_rng();
    println!("Random float: {}", (PI/6.0).sin());
    println!("Random float: {}", rng.gen::<f64>());
    let x: f32 = 2.0;
    let float_bytes = x.to_be_bytes();
    println!("Float: {:?}", float_bytes);

}