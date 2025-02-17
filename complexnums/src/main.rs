use num::complex::Complex;

fn main() {
    let a = Complex{re: 1.1, im: -1.2};
    let b = Complex::new(10.1, 22.2);
    let result = a + b;

    println!("{} + {}", result.re, result.im);
}
