use std::io::{stdin, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let server_addr = "127.0.0.1:2889";
    let mut socket = TcpStream::connect(server_addr).expect("can not connect to server");
    socket.set_nonblocking(true).expect("unexpecdted error");
    println!("{} connect",server_addr);
    start_thread(socket.try_clone().unwrap());

    let user = input("input name");
    loop{
        let msg = input("");
        let msg = format!("{}>{}\n",user,msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }


}

fn start_thread(socket: TcpStream){
    let mut reader = BufReader::new(socket);
    thread::spawn(move||loop{
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf){
            if n > 0{
                println!("[Receive Message] {}",buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
fn input(msg:&str)->String{
    if msg != ""{println!("{}",msg);}
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("input error");
    String::from(buf.trim())
}