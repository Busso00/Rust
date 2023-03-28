use std::sync::Arc;
use std::str;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field_byte:Vec<u8> = Vec::new();
    let mut v_res:Vec<String> = Vec::new();
    let mut nr=0;
    let mut nc=0;

    for row in minefield.iter() {
        for char in row.as_bytes() {
            field_byte.push(*char);
            if nr==0{
                nc+=1;
            }
            
        }
        nr+=1;
    }
    println!("{:?}",field_byte);
    for i in 0..field_byte.len(){
        if field_byte[i]==32{
            let irow=i/nc;
            let icol=i%nc;
            let mut count_adj=0;
            for j in 0..3{
                for k in 0..3{
                    let new_irow=(irow as i32)+(j as i32)-1;
                    let new_icol=(icol as i32)+(k as i32)-1;
                    if ((new_icol>=0)&&(new_icol<nc as i32))&&((new_irow>=0)&&(new_irow<nr as i32)){
                        if field_byte[(irow+j-1)*nc+icol+k-1]==42{
                            count_adj+=1;
                        }
                    }
                }
            }
            if count_adj!=0{
                field_byte[i]=count_adj+48;
            }
        }
    }
    for i in 0..nr{
        v_res.push(String::from(std::str::from_utf8(&field_byte[i*nc..(i+1)*nc]).unwrap()));
    }
    return v_res;
}

use clap::Parser;
/// Move your robot
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
///number of rows
    #[arg(short, long)]
    rows: usize,
///number of columns
    #[arg(short, long)]
    cols: usize,

    #[arg(short, long)]
    field: String,    
    
    #[arg(short, long, default_value_t = 4)]
    narg: u8,
}

fn main (){
    let args = Args::parse();
    let mut m_field:Vec<&str>=Vec::new();
    for i in 0..args.rows{
        m_field.push(&args.field[i*args.cols..(i+1)*args.cols]);
    }
    let marked_field=annotate(&m_field[..]);
    println!("{:?}", marked_field);
}
