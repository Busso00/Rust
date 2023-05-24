use std::{thread, time};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::io::{Read, Seek, Write, SeekFrom};
use std::fs::*;
use fcntl::{FcntlLockType, lock_file, unlock_file};
use std::thread;
use std::sync::{Arc, Mutex};



#[repr(C)]
#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}


#[repr(C)]
#[derive(Debug,Deserialize,Serialize,Clone,Copy)]
struct Buffer{
    full: bool,
    size: u64,
    first: u64,
    next: u64
}

#[repr(C)]
#[derive(Debug,Deserialize,Serialize,Clone)]
struct RingBuf{
    v: Vec<u8> 
}

impl RingBuf {
    fn read(&mut self, n: usize) -> Option<SensorData> {

    }
    fn write(&mut self, n: usize, data: &SensorData) -> Result<(),()>{

    }
}

fn main2( header2: Arc<Mutex<Buffer>>, ringBuf2: Arc<Mutex<RingBuf>>) {
    println!("p2 started...");

    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .open("data")
        .unwrap();

    //to determine the space occupied after serialization of struct serialize dummy data
    let target: Option<SensorData> = Some(SensorData{seq: 0, values: [0f32;10], timestamp: 0});
    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let m = (&encoded[1..]).len();
    let target: Option<Buffer> = Some(Buffer{full: false, size: 0, first: 0, next: 0});
    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let n = (&encoded[1..]).len();
    //debug output
    println!("size of SensorData: {} size of Buffer: {}",m,n);

    //debug code
    let mut last=0;

    loop {
        loop {
            //busy waiting for lock
            match lock_file(&f, None, Some(FcntlLockType::Write)) {
                Ok(true) => {

                    println!("Lock acquired!");

                    //read the header in order to know where to start reading (and where to stop)
                    let mut encoded = vec![0u8; n];
                    f.seek(SeekFrom::Start(0)).unwrap();
                    f.read(encoded.as_mut_slice()).unwrap();
                    let mut header: Buffer = bincode::deserialize(&mut encoded).unwrap();
                    //debug output
                    //println!("byte read header: {}", n);
                    //println!("{:?}", header);

                    //reading data
                    let full = if header.full{
                        1
                    }else{
                        0
                    };
                    let nval=(((header.next+header.size-header.first)%(header.size))+full*header.size) as usize; //need to add header.size to avoid overflow since first can be > than next
                    let mut data: Vec<SensorData> =Vec::new();
                    for i in 0..nval{
                        let mut encoded = vec![0u8; m*nval];//next is also last position
                        f.seek(SeekFrom::Start((n as u64) + (m as u64)*((header.first+(i as u64))%(header.size)))).unwrap();//first index starts at 0 end at heade.size-1
                        f.read(encoded.as_mut_slice()).unwrap();
                        let reading:SensorData= bincode::deserialize(&mut encoded).unwrap();
                        data.push(reading);
                    }
                    //debug output
                    println!("valid data: {:?}", data);
                    //println!("n record read: {}",nval);
                    //debug only code
                    if header.full{
                        let mut check_ten=data[0].seq;
                        assert_eq!(data[0].seq,last+1);
                        for i in 0..10{
                            assert_eq!(data[i].seq,check_ten);
                            check_ten+=1;
                        }
                    }
                    last=data[data.len()-1].seq;

                    //emptying circular buffer
                    header.full=false;
                    header.first=header.next;
                    let target: Option<Buffer>  = Some(header);
                    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
                    f.seek(SeekFrom::Start(0)).unwrap();
                    f.write(&encoded[1..]).unwrap();//1.. to throw away version byte
                    //debug output
                    //println!("byte written header: {}",n);
                    //println!("{:?}",&encoded[1..]);

                    match unlock_file(&f, None) {
                        Ok(true) => println!("Lock successfully released"),
                        Ok(false) => println!("Falied to release lock"),
                        Err(err) => println!("Error: {:?}", err),
                    }
                    break;//exit from busy waiting
                }
                Ok(false) => println!("Could not acquire lock!"),
                Err(err) => println!("Error: {:?}", err),
            }
        }

        {
            //sleep
            let ten_s = time::Duration::from_millis(10000);
            let now = time::Instant::now();

            thread::sleep(ten_s);
            assert!(now.elapsed() >= ten_s);
        }
    }
}



//producer
fn main() {
    println!("p1 started...");

    let mut rng = rand::thread_rng();
    let mut i:u32 = 0;

    let mut f = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open("data").unwrap();
    
    //to determine the space occupied after serialization of struct serialize dummy data
    let target: Option<SensorData> = Some(SensorData{seq: 0, values: [0f32;10], timestamp: 0});
    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let m = (&encoded[1..]).len();
    let target: Option<Buffer> = Some(Buffer{full: false, size: 0, first: 0, next: 0});
    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let n = (&encoded[1..]).len();
    //debug output
    println!("size of SensorData: {} size of Buffer: {}",m,n);

    //inizialization of the header
    let header = Arc::new(Mutex::new(Buffer{
        full: false,
        size: 20,//change here to 10/20
        first: 0,
        next: 0
    }));

    let ringBuf = Arc::new(Mutex::new(RingBuf{
            v: vec![52*20;0] //sizeof sensor data * 20
        }
    ));

    let mut header2 = header.clone();
    let mut ringBuf2 = ringBuf.clone();
    //spawn 2-nd thread
    thread::spawn( move || {
        return main2 (header2, ringBuf2);
    });

    //no need to write in file buffer header --> 2 separate data structure for data & header
    
    loop {
        //initialize struct
        let sd = SensorData {
            seq : i,
            values : rng.gen(),
            timestamp : 0//only to be more realistic, not required
        };
        //debug output
        println!("{:?}",sd);
        
        loop{//busy waiting for lock
            match lock_file(&f, None, Some(FcntlLockType::Write)) {
                Ok(true) => {

                    println!("Lock acquired!");
                   
                    //read the header in order to know where to start writing
                    
                    let mut encoded=vec![0u8;n];//mutable only when read
                    f.seek(SeekFrom::Start(0)).unwrap();
                    f.read(encoded.as_mut_slice()).unwrap();
                    let mut header : Buffer = bincode::deserialize(&mut encoded).unwrap();
                    //debug output
                    //println!("byte read header: {}",n);
                    //println!("{:?}",header);

                    //check buffer full
                    if !header.full {

                        //seek to correcprintln!("byte read data (some may be invalid): {}", m*20);t position according to next position
                        let target: Option<SensorData>  = Some(sd);
                        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
                        f.seek(SeekFrom::Start(header.next*(m as u64)+(n as u64))).unwrap();
                        f.write(&encoded[1..]).unwrap();//1.. to throw away version byte
                        //debug output
                        //println!("byte written data: {}",n);
                        //println!("{:?}",&encoded[1..]);

                        //update struct for header
                        header.next = (header.next+1)%header.size;
                        if header.next == header.first {
                            header.full = true; 
                        }

                        //propagate update of buffer to file header
                        let target: Option<Buffer>  = Some(header);
                        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
                        f.seek(SeekFrom::Start(0)).unwrap();
                        f.write(&encoded[1..]).unwrap();//1.. to throw away version byte
                        //debug output
                        //println!("byte written header: {}",n);
                        //println!("{:?}",&encoded[1..]);
                    }

                    match unlock_file(&f, None) {
                        Ok(true) => println!("Lock successfully released"),
                        Ok(false) => println!("Falied to release lock"),
                        Err(err) => println!("Error: {:?}", err),
                    }

                    break;//exit from busy waiting 
                },
                Ok(false) => println!("Could not acquire lock!"),
                Err(err) => println!("Error: {:?}", err)
            }
        }
    
        {//sleep
            let one_s = time::Duration::from_millis(1000);
            let now = time::Instant::now();
            thread::sleep(one_s);
            assert!(now.elapsed() >= one_s);
        }

        i+=1;

    }
}
