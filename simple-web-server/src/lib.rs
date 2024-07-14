use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

/// 线程池
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// 工作者
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// 线程池实现
impl ThreadPool {
    /// 创建一个线程池
    ///
    /// `size`: 线程池的数量
    ///
    /// # Panics
    ///
    /// new 函数在 size 为 0时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }
        return ThreadPool {
            workers,
            sender: Some(sender),
        };
    }

    /// 在另外一个线程中， 执行 `f` 回调
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.as_ref() {
            None => {}
            Some(sender) => sender.send(job).unwrap(),
        }
        // self.sender.send(job).unwrap()
    }
}

/// 处理ThreadPool销毁，保证线程池中线程都已执行完
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 销毁发送者，这样 worker中的接收者会接收到一个异常
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            //recv 会阻塞当前线程
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("worker {id} receive job");
                    job()
                }
                Err(_) => {
                    println!("worker {id} thread end");
                    break;
                }
            }
            // let job = receiver.lock().unwrap().recv().unwrap();
            // job();
        });
        // 使用 while let 或 if let或 match 直到相关的代码块结束都才会丢弃临时值， 即lock()的锁在 job调用期间一直持续，其他 worker无法接受到任务
        // while let Ok(job) = receiver.lock().unwrap().recv() {
        //     println!("Worker {id} got a job; executing.");
        //
        //     job();
        // }
        return Worker {
            id,
            thread: Some(thread),
        };
    }
}
