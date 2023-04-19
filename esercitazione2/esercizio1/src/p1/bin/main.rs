use std::{thread, time};
use rand::Rng;
use serde::{Serialize,Deserialize};
use std::io::{Read, Seek, Write, SeekFrom};
use std::fs::*;
use fcntl::{FcntlLockType, lock_file, unlock_file};

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
    let header = Buffer{
        full: false,
        size: 10,//change here to 10/20
        first: 0,
        next: 0
    };

    let target: Option<Buffer>  = Some(header);
    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();
    f.write(&encoded[1..]).unwrap();//1.. to throw away version byte
    //debug output
    //println!("byte written {}",n);
    //println!("{:?}",&encoded[1..]);
    
    loop {
        //initialize struct
        let sd = SensorData {
            seq : i,
            values : rng.gen(),
            timestamp : //only to be more realistic, not required
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
