/* Rust PI constant --> std::f64::consts::PI;
 * Convert degrees to radians --> radians = degrees * PI / 180.0;
 * Allication 2D vecotr -> let vector2d: Vec<Vec<f64>> = vec![vec![0.0; 2]; 10];
 */

 mod multisine;

 use std::io::prelude::*;
 use std::fs::File;

fn main() -> std::io::Result<()> {
    //let vector2d: Vec<Vec<f64>> = vec![vec![0.0; 2]; 10];
    //let mut vector2d: [[f64; 2]; 10] = [[0.0; 2]; 10];
    //vector2d[2][1] = 1.0;
    //println!("{:?}", vector2d);
    println!("Hello, world!");
    let mut file = File::create("multisine.dat")?;
    let multisine_samples = multisine::create_discrete_multisine(1, 10);
    file.write_all(multisine_samples.to_be)?;
    Ok(())

}
