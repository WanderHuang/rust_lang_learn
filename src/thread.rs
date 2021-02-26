use std::thread::{spawn, JoinHandle};
use std::time;
use std::{
    sync::{mpsc::channel, mpsc::Sender, Arc, Mutex},
    thread::sleep,
};

pub fn thread_test() {
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
            println!("- {}", x);
            sender_1.send(x).unwrap();
        }));

        // 线程二
        let mutex = Arc::clone(&store);
        threads.push(spawn(move || {
            let mut vec = mutex.lock().unwrap();
            vec.push(x);
            println!("+ {}", x);
            sender_2.send(x).unwrap();
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("store {:?}", store);

    for x in receiver {
        println!("receive > {}", x);
    }
}
