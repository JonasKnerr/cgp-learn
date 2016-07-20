use std::fmt;
struct Swagger<T: fmt::Display> {
    a: T,
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(#swag {} #yolo)", self.a)
    }
}

fn main() {
    let test = Swagger { a: 32 };
    println!("{}", test);
}
