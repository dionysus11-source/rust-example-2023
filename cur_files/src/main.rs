use std::{env, path};

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >=2{
        target_dir = &args[1];
    }
    let target = path::PathBuf::from(target_dir);
    tree(&target,0);
}

fn tree(dir:&path::PathBuf, depth:isize){
    let files = dir.read_dir().expect("not valid directory");
    for file in files{
        let file = file.unwrap().path();
        for i in 1..=depth{
            print!("| ");
        }
        let fname = file.file_name().unwrap().to_string_lossy();
        println!("{}",fname);
        if file.is_dir(){
            tree(&file, depth+1);
            continue;
        }

    }
}
