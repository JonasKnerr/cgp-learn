use std::iter::Iterator;
struct Fib {
    curr: u32,
    next: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new = self.curr + self.next;
        self.curr = self.next;
        self.next = new;
        Some(self.curr)
    }
}
impl Fib {
    fn new() -> Fib {
        Fib { curr: 1, next: 1 }
    }
}



fn main() {
    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}
