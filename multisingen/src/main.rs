/* Rust PI constant --> std::f64::consts::PI;
 * Convert degrees to radians --> radians = degrees * PI / 180.0;
 */

use num::Complex;
//use std::fs;

//use std::io;
use std::io::prelude::*;
//use std::fs::File;
use std::fs::OpenOptions;

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let pi = std::f64::consts::PI;
    let a = (pi/180.0*30.0).sin();
    println!("Sin(30) = {}", a);

    let complex_float = Complex::new(10.1, 20.1);

    println!("Complex float: {}", complex_float);

    let point = Point { x: 1, y: 2 };
    let mut f = OpenOptions::new().read(true).write(true).create(true).open("foo.txt")?;

    f.write_all(&point)?;
    let mut b: Complex<f64> = Complex::new(0.0, 0.0);
    //fs::read("numbers.dat", &b);
    println!("Complex float: {}", b);

}
