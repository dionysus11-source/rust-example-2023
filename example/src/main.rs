use std::env;
use std::fs;

fn main(){
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("no file");
    return;
    }
    let filename = &args[1];
    let text = match fs::read_to_string(filename){
        Ok(fs) => fs,
        Err(e)=> e.to_string(),
    };
    let lines = text.split('\n');
    let mut total:f64 = 0.0;
    for line in lines{
        let n:f64 = match line.parse(){
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += n;
    }
    println!("total is {}", total);
    
}