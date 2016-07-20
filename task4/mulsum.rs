use std::ops::Add;
use std::ops::Mul;
fn main() {
    println!("{:?}", mul_sum(4, 5));
}
fn mul_sum<T: Mul + Add + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {

    return (a + b, a * b);
}
