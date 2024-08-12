use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // 创建第1个子线程执行worker
    let rx1 = rx.clone();
    let handle1 = thread::spawn(move || {worker(rx1);});
    thread::sleep(Duration::from_secs(5));
    tx.send(0).unwrap();
    // 等待第1个子线程结束
    handle1.join().unwrap();

    let rx2 = rx.clone();
    let handle2 = thread::spawn(move || {worker2(rx2);});
    thread::sleep(Duration::from_secs(5));
    tx.send(1).unwrap();
    // 等待第2个子线程结束
    handle2.join().unwrap();

    println!("主线程退出...");
}

fn worker(rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        // 锁定 Mutex 并尝试接收消息
        let msg = {
            let rx = rx.lock().unwrap();
            rx.try_recv()
        };

        match msg {
            Ok(_) => {
                println!("worker 线程接收到终止信号，准备退出...");
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("worker 线程执行任务...");
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("通道断开, worker 线程退出...");
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn worker2(rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        // 锁定 Mutex 并尝试接收消息
        let msg = {
            let rx = rx.lock().unwrap();
            rx.try_recv()
        };

        match msg {
            Ok(_) => {
                println!("worker2 线程接收到终止信号，准备退出...");
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("worker2 线程执行任务...");
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("通道断开, worker2 线程退出...");
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}