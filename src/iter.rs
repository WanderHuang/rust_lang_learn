use std::{mem, usize};
// use std::collections::LinkedList;

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

/// 一个链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    val: Option<T>,
    next: Option<Box<Node<T>>>,
}

/// 一个链表
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

/// 链表迭代器
struct LinkedListIter<T> {
    curr: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Clone {
  fn as_list(&mut self) -> LinkedList<T> {
    LinkedList::new(self)
  }
}

impl<T> LinkedList<T> where T: Clone {
    fn new(head: &mut Node<T>) -> Self {
        Self {
            head: Some(Box::new(head.clone())),
        }
    }
    fn iter(self) -> LinkedListIter<T> {
        LinkedListIter { curr: self.head }
    }
}

impl<T> Iterator for LinkedListIter<T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match mem::replace(&mut self.curr, None) {
            Some(node) => {
                self.curr = node.next;
                match &mut self.curr {
                  Some(curr) => mem::replace(&mut curr.val, None),
                  None => None
                }
            }
            None => None,
        }
    }
}

pub fn iter_test() {
    let mut fib = Fibonacci::new();
    let nth = 4;
    // value : 1 1 2 3 5 8 13
    // nth   : 0 1 2 3 4 5 6
    println!("{} nth: {:?}", nth + 2, fib.nth(nth));

    println!(
        "take 4: {:?}",
        Fibonacci::new().take(4).collect::<Vec<u32>>()
    );

    println!(
        "skip 4: {:?}",
        Fibonacci::new().skip(4).take(4).collect::<Vec<u32>>()
    );

    let mut head = Node {
        val: Some(0),
        next: None,
    };
    let mut a = Node {
        val: Some(1),
        next: None,
    };
    let mut b = Node {
        val: Some(2),
        next: None,
    };
    let c = Node {
        val: Some(3),
        next: None,
    };

    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));
    head.next = Some(Box::new(a));

    let iter = head.as_list().iter();
    let iter2 = head.as_list().iter();

    for x in iter {
        println!("1st > {}", x);
    }
    for x in iter2 {
        println!("2nd > {}", x);
    }
}
