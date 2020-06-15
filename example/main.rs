use ftfp::*;

fn main() {
    let a = Fixed::from(2.);
    let b = Fixed::from(5.1234932400);
    let out = a - b;
    println!("out {:.3}", out);
    let out: f64 = out.into();
    println!("out {}", out);
    println!("bits {}", FIX_INT_BITS);
}
