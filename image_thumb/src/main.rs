use image::{self, imageops, GenericImageView};
use std::sync::mpsc;
fn main() {
    println!("Hello, world!");
    let size = 128;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{
        println!("[USAGE] image_thumb imagefile");
        return;
    }
    let infile = String::from(&args[1]);
    let file_name: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}-thumb.png",file_name[0]);
    println!("input : {}", infile);
    println!("output: {}", outfile);
    let mut img= image::open(infile).expect("Can not load file");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};
   // let mut img2 = imageops::crop(&mut img,(dim.0-w)/2,(dim.1-w)/2,w,w).to_image();
let mut img2 = if dim.0 > dim.1 {
    imageops::crop(&mut img,(dim.0-dim.1)/2,0,dim.1,dim.1).to_image()
} else {
    imageops::crop(&mut img,0,(dim.1-dim.0)/2,dim.0,dim.0).to_image()
};

    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    img3.save(outfile).unwrap();

    let (tx, rx) = mpsc::channel::<(i64,i64)>();
    rx.recv()

}
