use std::cmp::Ordering;
use std::io::Write;
use std::io::stdin;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::io::{Error, ErrorKind};

static list_cmd_no_output: [&str;3] = ["cd", "ls", "echo"];
static list_cmd_output: [&str;2] = ["cat", "rev"];


enum Event {
    InputLine(String),
    OutputLine(Result<ExitStatus, std::io::Error>),
    End,
    Done
}

fn start_child(rx: Receiver<Event>, tx: SyncSender<Event>) {
    let mut curr_child= Command::new("sh")
    .arg("-c")
    .spawn()
    .expect("child command failed");
    curr_child.kill().unwrap();

    loop {
        match rx.recv() {
            Ok(Event::InputLine(cmd)) => {
                let cmd_trim = String::from(cmd.trim());
                //println!("child start process...");
                curr_child = Command::new("sh")
                    .arg("-c")
                    .arg(cmd_trim.clone())
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("child command failed");
                
                let mut found = false;
                /*
                for cmd_i in list_cmd_no_output{ //not necessary
                    if cmd_trim == String::from(cmd_i){
                        //println!("enter waiting ...");
                        found=true;
                        let res = curr_child.wait();
                        //std::io::stdout().flush().unwrap();
                        tx.send(Event::OutputLine(res));
                    }
                }
                */
                for cmd_i in list_cmd_output{
                    if cmd_trim == String::from(cmd_i){
                        found=true;
                        tx.send(Event::Done).unwrap();
                    }
                }
                if !found{
                    let res = curr_child.wait();
                    //std::io::stdout().flush().unwrap();
                    tx.send(Event::OutputLine(res));
                }
            }
            Ok(Event::End) => {
                
            },
            Ok(Event::Done) =>{
                let byte: char = '^';
                let str = byte.to_string();
                let mut buf = String::new();
                //println!("reading input...");
                let mut my_stdin = curr_child.stdin.take().unwrap();
                while buf.trim() != str{
                    buf = String::new();
                    std::io::stdin().read_line(&mut buf).unwrap();
                    my_stdin.write_all(buf.as_bytes()).unwrap();
                    //println!("buffer is {}", buf.trim());
                    //println!("reading...");
                }
                curr_child.kill().unwrap();
                //std::io::stdout().flush().unwrap();
                tx.send(Event::End).unwrap();
            },
            _ => {}
        }
    }
}

fn start_user(tx: SyncSender<Event>, tx2: SyncSender<Event>) {
    //tx2.send(Event::InputLine("0".to_string())).unwrap();
    loop {
        let mut cmd = String::new();
        tx2.send(Event::InputLine("0".to_string())).unwrap();
        //std::io::stdout().flush().unwrap();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut cmd).unwrap();
        tx.send(Event::InputLine(cmd)).unwrap();
    }
}

fn main() {
    let (tx1, rx1) = sync_channel(1);
    let (tx2, rx2) = sync_channel(1);
    let (tx3,rx3)=sync_channel(1);
    let tx2c = tx2.clone();

    //println!("hello world!");
    thread::spawn(move || start_child(rx1, tx2));
    thread::spawn(move || start_user(tx2c, tx3));
    //rx3.recv().unwrap(); //skip first message

    

    loop {
        
        match rx2.recv().unwrap() {
            Event::InputLine(input) => {
                //println!("input received by loop...");
                tx1.send(Event::InputLine(input)).unwrap();
            },
            Event::OutputLine(output) => {
                //println!("process ended with output: {:?}", output);
                rx3.recv().unwrap();//unlock
                //println!("loop unlocked 1...");
            },
            Event::Done =>{
                //println!("process started... terminate with CTRL+A");
                tx1.send(Event::Done).unwrap();
            }
            Event::End=> {
                //println!("terminated");
                tx1.send(Event::End).unwrap();
                rx3.recv().unwrap();//unlock
                //println!("loop unlocked 2...");
            }
        }
    }
}
