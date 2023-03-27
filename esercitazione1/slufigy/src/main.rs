
const SUBS_I : &str =
"àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str =
"aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn conv(c: char) -> char {
    let vI:Vec<char>=SUBS_I.chars().collect();
    let vO:Vec<char>=SUBS_O.chars().collect();
    let mut c_ret=char::from(c);
    for i in 0..vI.len(){
        if c==vI[i]{
            c_ret=vO[i];
            break;
        }
    }
    c_ret.to_lowercase();
    if !c_ret.is_alphanumeric(){
        c_ret='-';
    }
    return c_ret;
}

fn slufigy(s: &str) -> String {
    let s_string=String::from(s);
    let mut s_res =String::new();
    let mut v: Vec<char> = s_string.chars().collect();
    let mut last_c='0';

    for mut c in v {
        c=conv(c);
        if !((last_c == c) && (c=='-')){
            s_res.push(c);
        }
        last_c=c;
    }
    if last_c=='-'{
        if s_res.len()!=1 {
            s_res=String::from(&s_res[0..s_res.len()-1]);
        }
    }
    println!("{}",s_res);
    return s_res;
}

use clap::Parser;
/// Convert a string to a slug
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
/// String to slufigy
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {

    slufigy("ciaoèéà°à+aoa  suus sas òç  ");
    slufigy("+-*/ ");

    let args = Args::parse();
    for _ in 0..args.count{
        slufigy(args.name.as_str());
    }
}
