use std::thread::{spawn, JoinHandle};
use std::time::{self, Duration};
use std::{
    sync::{mpsc::channel, mpsc::Sender, Arc, Mutex},
    thread::sleep,
};

pub fn thread_test() {
    // Arc: Atomically Reference Counted
    // 自动引用计数
    // mutex 互斥锁
    let store = Arc::new(Mutex::new(vec![]));

    let (sender, receiver) = channel();

    let mut threads: Vec<JoinHandle<()>> = vec![];

    for x in 0..10 {
        // sender 多线程使用需要clone为多份
        let sender_1 = Sender::clone(&sender);
        let sender_2 = Sender::clone(&sender);

        // 线程一
        let mutex = Arc::clone(&store);
        threads.push(spawn(move || {
            let mut vec = mutex.lock().unwrap();
            vec.push(x);
            println!("线程 🍎 -> {}", x);
            sender_1.send(x).unwrap();

            sleep(Duration::from_millis(1000));
            drop(sender_1);
        }));

        // 线程二
        let mutex = Arc::clone(&store);
        threads.push(spawn(move || {
            let mut vec = mutex.lock().unwrap();
            vec.push(x);
            println!("线程 🍍 -> {}", x);
            sender_2.send(x).unwrap();
            sleep(Duration::from_millis(2000));
            drop(sender_2);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    // println!("store {:?}", store);
    println!("Result: {:?}", *store.lock().unwrap());

    // 所有sender都drop后，receiver自动关闭，channel自动关闭
    drop(sender);

    for x in receiver {
        println!("receive > {}", x);
    }

    
}
