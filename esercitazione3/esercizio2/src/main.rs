use std::{thread, time};
use rand::Rng;
use std::sync::{Arc, Mutex};



#[repr(C)]
#[derive(Debug,Clone,Copy)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}


#[repr(C)]
#[derive(Debug,Clone,Copy)]
struct Buffer{
    full: bool,
    size: usize,
    first: usize,
    next: usize
}

#[repr(C)]
#[derive(Debug,Clone)]
struct RingBuf{
    v: Vec<SensorData> 
}

impl RingBuf {
    fn read(header:Arc<Mutex<Buffer>>, ring_buf:Arc<Mutex<RingBuf>>) -> Option<SensorData> {
        let mut curr_header=header.lock().unwrap();
        let curr_buffer=ring_buf.lock().unwrap();

        if curr_header.full{
            //avoid nevagive number in op %
            let res = curr_buffer.v[curr_header.first];
            curr_header.first = ( curr_header.first + 1 ) % curr_header.size;
            curr_header.full = false;

            return Some(res);

        }else if curr_header.first!=curr_header.next {
            let res = curr_buffer.v[curr_header.first];
            curr_header.first = ( curr_header.first + 1 ) % curr_header.size;

            return Some(res);
        }else{
            //nothing to read
            return None;
        }

    }

    fn write(header:Arc<Mutex<Buffer>>, ring_buf:Arc<Mutex<RingBuf>>, data: &SensorData) -> Result<(),()>{
        let mut curr_header=header.lock().unwrap();
        let mut curr_buffer=ring_buf.lock().unwrap();
       

        if !curr_header.full {
            curr_buffer.v[curr_header.next]=*data;

            curr_header.next = ( curr_header.next + 1 ) % curr_header.size;
            if curr_header.next == curr_header.first {
                curr_header.full = true; 
            }
            return Ok(());
        }
        return Err(());    
    }
}

//consumer
fn main2( header2: Arc<Mutex<Buffer>>, ring_buf2: Arc<Mutex<RingBuf>>) {
    println!("p2 started...");

    //debug code

    loop {
        {
            //sleep
            let ten_s = time::Duration::from_millis(10000);
            let now = time::Instant::now();

            thread::sleep(ten_s);
            assert!(now.elapsed() >= ten_s);
        }
        //no need to do busy wait with lock
        {
            let mut v : Vec<SensorData> = Vec::new();
            loop{
                match RingBuf::read(header2.clone(),ring_buf2.clone()){
                    Some(res)=>{
                        
                        v.push(res);
                    },
                    None => break
                }
            }
            println!("consumer: Receive values: {:?}", v);
        }
        
    }
}

//producer
fn main() {
    println!("p1 started...");

    let mut rng = rand::thread_rng();
    let mut i:u32 = 0;

    //inizialization of the header
    let header = Arc::new(Mutex::new(Buffer{
        full: false,
        size: 10,//change here to 10/20
        first: 0,
        next: 0
    }));

    let ring_buf = Arc::new(Mutex::new(RingBuf{
            v: vec![SensorData{
                seq:0,
                values: [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],
                timestamp: 0
            };20] //sizeof sensor data * 20
        }
    ));

    let header2 = header.clone();
    let ring_buf2 = ring_buf.clone();
    //spawn 2-nd thread
    thread::spawn( move || {
        return main2 (header2, ring_buf2);
    });

    //no need to write in file buffer header --> 2 separate data structure for data & header
    //use 
    

    loop {
        //initialize struct
        let sd = SensorData {
            seq : i,
            values : rng.gen(),
            timestamp : 0//only to be more realistic, not required
        };
        //no need to do busy wait with lock
        println!("producer: Send value: {:?}",sd);
        RingBuf::write(header.clone(), ring_buf.clone(), &sd).unwrap();

        
        {//sleep
            let one_s = time::Duration::from_millis(1000);
            let now = time::Instant::now();
            thread::sleep(one_s);
            assert!(now.elapsed() >= one_s);
        }

        i+=1;

    }
}
