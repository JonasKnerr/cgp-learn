use std::ops::{Add, Mul};

fn main() {
    println!("{:?}", mul_sum(4, 5));
}
fn mul_sum<T: Mul + Add + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {

    (a + b, a * b)
}
