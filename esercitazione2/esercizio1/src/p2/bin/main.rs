use fcntl::{lock_file, unlock_file, FcntlLockType};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{thread, time};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32,
}

#[repr(C)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
struct Buffer {
    full: bool,
    size: u64,
    first: u64,
    next: u64,
}

fn main() {
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
