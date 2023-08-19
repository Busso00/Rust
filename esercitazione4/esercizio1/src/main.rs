
use std::sync::{Arc, Mutex, Condvar};

struct CyclicBarrier{
    pair: (Mutex<(u8,u8)>,Condvar),
    n: u8
}

impl CyclicBarrier{

    fn new(n: u8) -> CyclicBarrier{
        return CyclicBarrier{
            pair: (Mutex::new((0,0)),Condvar::new()),
            n: n
        };
    }

    fn wait(&self){
        let (lock, cvar) = &self.pair;
        let mut v = lock.lock().unwrap();
        
        v.0+=1;//# waiting threads grows
        //println!("growing...");
        v=cvar.wait_while(v, |v|
        {
            return v.0<self.n && v.1==0;//while v.0<#threads and phase == growing(v.1=0) wait since all threads arrived
            //v.1 is useful to make all threads next to the first with v.0==self.n continue (break waiting)
            //otherwise the first to exit can set v.0-=1 and re-block other threads causing deadlock
            //(wait_while doesn't acquire v at notify time but when checked)
        }
        ).unwrap();

        cvar.notify_all();//last thread doesn't wait start next thread
        v.1=1;//true: all thread arrived -> start shrinking phase
        v.0-=1;//# waiting threads diminishes
        //println!("shrinking...");
        v=cvar.wait_while(v, |v|
        {
            return v.0>0 && v.1==1;//while v.0>0 and phase == shrinking(v.1=1) wait since all threads exited from growing(waiting) phase
            //v.1 is useful to make all threads next to the first with v.0==0 continue (break waiting)
            //otherwise the first to exit can do next loop (external to wait) set v.0+=1 and re-block other threads causing deadlock
            //(wait_while doesn't acquire v at notify time but when checked)
        }
        ).unwrap();
        
        v.1=0;
        
    }
}

use crossbeam::channel::{Sender, Receiver, bounded};

struct CyclicBarrierChannnel {
    vchan: Vec<(Sender<u8>,Receiver<u8>)>,
    n: usize
}

impl CyclicBarrierChannnel {

    fn new(n:usize) -> CyclicBarrierChannnel {
        let mut vchan = Vec::new();
        
        for _ in 0..n{
            vchan.push(crossbeam::channel::bounded::<u8>(n-1));
                    }
        return CyclicBarrierChannnel{vchan: vchan, n: n};
    }
    
    fn wait(&self, i: u8){
        for nc in 0..self.n{
            if nc != i as usize{
                self.vchan[nc].0.send(0).unwrap();//signal that thread 0 is here
            }
        }
        //let mut count=0;
                    
                for nc in 0..self.n{
            if nc==i as usize{
                continue;
            }
            self.vchan[i as usize].1.recv().unwrap();
        }
    }
}


use std::thread;
struct CyclicBarrierThread {
    vchan: Vec<(Sender<u8>,Receiver<u8>)>,
    n: usize
}

impl CyclicBarrierThread {
    fn new(n:usize) -> CyclicBarrierThread {
        let mut vchan = Vec::new();

        for _ in 0..n*2{
            vchan.push(crossbeam::channel::bounded::<u8>(1));
        }
        let vchan_c = vchan.clone();
        thread::spawn(move ||{
            loop {
                for i in n..n*2{
                    vchan_c[i].1.recv().unwrap();
                }
    
                for i in 0..n{
                    vchan_c[i].0.send(0).unwrap();
                }
            }
        });
        return Self{vchan: vchan, n:n}
    }

    fn wait(&self, i: u8){
        
        self.vchan[i as usize+ self.n].0.send(0).unwrap();

        self.vchan[i as usize].1.recv().unwrap();//signal that thread 0 is here
    
    }
}

fn main() {
    let abarrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let mut cbarrier = abarrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }

    let abarrier = Arc::new(CyclicBarrierChannnel::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let mut cbarrier = abarrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait(i);
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }

    let abarrier = Arc::new(CyclicBarrierThread::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let mut cbarrier = abarrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait(i);
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }

}
