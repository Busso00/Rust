use std::{thread, thread::JoinHandle};

use clap::Parser;

fn calculate_permutations_rep_r(
    v: &mut Vec<i32>,
    pos: usize,
    max: usize,
    values_v: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    let mut last = 0;
    if pos == max {
        result.push(v.clone());
        return;
    }

    for i in 0..(max - pos) {
        if (i > 0) && (last == values_v[i]) {
            continue;
        }
        v[pos] = values_v[i];
        let mut pass_v = values_v.clone();
        pass_v.remove(i);
        calculate_permutations_rep_r(v, pos + 1, max, &mut pass_v, result);
        last = v[pos];
    }
}

//wrapper of recursive function
fn calculate_permutations_rep_w(values_v: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    //wrapper function sort and instantiate vector
    let max = values_v.len();
    values_v.sort();
    println!("{:?}", values_v);
    let mut v: Vec<i32> = vec![];
    for _ in 0..max {
        v.push(0);
    }
    calculate_permutations_rep_r(&mut v, 0, max, values_v, result);
}


fn calculate_variations_rep_r(
    v: &mut Vec<usize>,
    pos: usize,
    max: usize,
    values_v: &Vec<i32>,
    result: &mut Vec<String>,
) {
    if pos == max {
        let mut res = values_v[0];
        for i in 0..max {
            match v[i] {
                0 => res += values_v[i + 1],
                1 => res -= values_v[i + 1],
                2 => res *= values_v[i + 1],
                3 => res /= values_v[i + 1],
                _ => unreachable!(),
            }
        }
        if res == 10 {
            let mut s = values_v[0].to_string();
            for i in 0..max {
                let op = match v[i] {
                    0 => "+",
                    1 => "-",
                    2 => "*",
                    3 => "/",
                    _ => unreachable!(),
                };
                s = format!("{}{}{}", s, op, values_v[i + 1].to_string());
            }
            result.push(s);
        }
        return;
    }
    for i in 0..max {
        //iterate over all possible operations
        v[pos] = i;
        calculate_variations_rep_r(v, pos + 1, max, values_v, result);
    }
}

//wrapper of recursive function
fn calculate_variations_rep_w(values_v: &Vec<i32>, result: &mut Vec<String>) {
    let mut v: Vec<usize> = vec![];
    for _ in 0..values_v.len()-1 {
        v.push(0);
    }
    calculate_variations_rep_r(&mut v, 0, values_v.len()-1, values_v, result);
}

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    ///values to permute
    #[arg(short, long)]
    val: String,
}

fn thread_entrypoint<'a>(values_p: Box<Vec<Vec<i32>>>) -> Box<Vec<String>>{
    let mut thread_result: Box<Vec<String>>= Box::new(vec![]);
    
    for value_p in *values_p {
        calculate_variations_rep_w( & value_p, &mut (*thread_result));
    }
    
    return thread_result;
}

fn main() {
    let args = Args::parse();
    let values_s = args.val.parse::<String>().unwrap();
    let mut values_v: Vec<i32> = values_s
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut values_p: Vec<Vec<i32>> = vec![];
    let mut result: Vec<String> = vec![];
    calculate_permutations_rep_w(&mut values_v, &mut values_p);

    let type_execution: i32 = 1;

    if type_execution == 0 {
        println!("execution mode: 0");
        for value_p in values_p {
            calculate_variations_rep_w( &value_p, &mut result);
        }

    } else if type_execution == 1 {
        println!("execution mode: 1");
        let n_thread = 4;//change also thread_data dimension
        let mut threads: Vec<JoinHandle<Box<Vec<String>>>> = vec![];
        //check if workload is equally divisible
        if values_p.len() % n_thread == 0 {

            for i in 0..n_thread {

                let mut thread_data: Box<Vec<Vec<i32>>> = Box::new(vec![vec![];5]);//change also n_thread
                //clone the necessary data
                thread_data.clone_from_slice(&values_p[(values_p.len() / n_thread) * i..(values_p.len() / n_thread) * (i + 1)]);
                //start the i-th thread
                threads.push(thread::spawn( move || {
                    return thread_entrypoint(thread_data);
                }));
            }
        for t in threads {
            //wait for thread finishing and save result
            let partial_res = t.join().unwrap();
            //push new solutions into result
            for s in *partial_res{
                result.push(s);
            }
        }
        } else {
            eprint!("values_p must be multiple of nthreads");
        }
    }

    println!("{:?} of length {}", result, result.len());
    /*generate all permutations of el in values_v
     * pseudocode:
     * result=[]
     * values_v.sort();
     * calculate_permutation(v, pos, max, values_v, result){
     *   last=0;
     *   if (pos == max){
     *     result.push(v);
     *   }
     *   for(i=0;i<max;i++){
     *     if(i>0 && last==values_v[i])
     *       continue;
     *     calculate_permutation(v, pos+1, max, v[pos] = values_v.pop(i), result)
     *     last = v[pos];
     *   }
     * }
     */
    /* perform all possible operations in all orders (variations w rep of 4 elements add,sub,mul,div)
     * pseudocode:
     * result=[];
     * calculate_variations(v, pos, max, values_v, result){
     *   if(pos == max){
     *     res=values_v[0];
     *     for(i=0; i<max; i++){
     *       switch(v[i]){
     *         case 0:
     *           res+=values_v[i+1];
     *         case 1:
     *           res-=values_v[i+1];
     *         case 2:
     *           res*=values_v[i+1];
     *         case 3:
     *           res/=values_v[i+1];
     *       }
     *     }
     *     if(res==10){
     *       result.push(values_v[0]+v[0]+values_v[1]+...);
     *     }
     *   }
     *   for (i=0;i<max;i++){//iterate over all possible operations
     *     v[pos] = i;
     *     rec(v, pos+1 , 4, values_v, result);
     *    }
     * }
     */
}
