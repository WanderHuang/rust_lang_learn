use std::usize;

/// 斐波那契数列，index从零开始
///
/// ```
/// fib.take(4) // 获取第5个;
/// ```
#[derive(Debug)]
struct Fibonacci {
    prev: u32,
    curr: u32,
    index: usize,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let next = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = next;
        Some(next)
    }
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            prev: 0,
            curr: 1,
            index: 0,
        }
    }
}

pub fn iter_test() {
    let mut fib = Fibonacci::new();
    let nth = 4;
    // value : 1 1 2 3 5 8 13
    // nth   : 0 1 2 3 4 5 6
    println!("{} nth: {:?}", nth + 2, fib.nth(nth));

    println!("take 4: {:?}", Fibonacci::new().take(4).collect::<Vec<u32>>());

    println!("skip 4: {:?}", Fibonacci::new().skip(4).take(4).collect::<Vec<u32>>());
}
