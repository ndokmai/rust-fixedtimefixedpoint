use ftfp::*;

fn main() {
    let a = Fixed::from(2.);
    let b = Fixed::from(5.);
    let out = a - b;
    let out: f64 = out.into();
    println!("out {}", out);
}
