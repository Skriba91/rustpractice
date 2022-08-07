use gnuplot::{Figure, Caption, Color};

fn main() {
    println!("Hello, world!");

    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    println!("x: {:?}, y: {:?}", x, y);
    let mut fg = Figure::new();
    fg.axes2d()
    .lines(&x, &y, &[Caption("A line"), Color("black")]);
    fg.show().unwrap();
}
