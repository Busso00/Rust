use std::thread;
use std::sync::mpsc::{channel,sync_channel,Sender,SyncSender,Receiver};
use std::time::Duration;

struct ThreadPool<F: FnOnce()->() + Send + 'static>{
    //thread_v: Vec<JoinHandle<usize>>
    n: usize,
    channel_v: Vec<SyncSender<F>>,
    channel_finish: Receiver<usize>
}



impl<F: FnOnce()->() + Send + 'static> ThreadPool<F> {

    fn execute_loop(rx1: Receiver<F>, tx2: SyncSender<usize>, tid: usize){
        loop{
            tx2.send(tid).unwrap();
            if let Ok(fun) = rx1.recv(){
                fun();
            }
        }
    }


    pub fn new(nthread: usize) -> Self{
        let (tx2,rx2)= sync_channel(nthread);
        //let this = &mut Self{n: nthread, channel_v:Vec::new(), channel_finish: rx2};
        let mut channel_v = Vec::new();
        for i in 0..nthread{
            let (tx1,rx1)= sync_channel(1);
            let tx2c = tx2.clone();
            channel_v.push(tx1);
            //this.thread_v.push(thread::spawn(move || this.execute_loop(rx1,tx2c,i)));
            thread::spawn(move || ThreadPool::execute_loop(rx1,tx2c,i));
        }
        return Self{n:nthread, channel_v:channel_v, channel_finish:rx2};
    }

    pub fn execute(&self, job: F) {
        if let Ok(free_tid) = self.channel_finish.recv(){
            self.channel_v[free_tid].send(job).unwrap();
        }
    }
}

fn main() {
    // alloca i worker
    let threadpool = ThreadPool::new(10);
    for x in 0..100 {
        threadpool.execute(move || {
        println!("long running task {}", x);
        thread::sleep(Duration::from_millis(1000))
        })
    }
    // just to keep the main thread alive
    loop {thread::sleep(Duration::from_millis(1000))};
}