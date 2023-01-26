use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn main() {
    let server_addr = "127.0.0.1:2889";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_addr).expect("Server error");
    server.set_nonblocking(true).expect("unknow error");
    println!("{} start server! ",server_addr);
    loop{
        if let Ok((client, addr)) = server.accept(){
            println!("Client is connected. {}",addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }
        if let Ok(msg) = rx.try_recv(){
            println!("Send to all : {}",msg.trim());
            clients= send_all(clients, &msg);
        }
    }
}

fn start_thread(client:TcpStream, tx:mpsc::Sender<String>){
    let mut reader = BufReader::new(client);
    thread::spawn(move|| loop{
        let mut msg = String::new();
        let result = reader.read_line(&mut msg);
        match result{
            Err(e) => {
                match e.kind(){
                    std::io::ErrorKind::ConnectionReset =>{println!("client is disconnected");break;}
                    _ => {}
                }
            },
            Ok(n) => {if n> 0{tx.send(msg).unwrap()}},
        }

        //if let Ok(n) = reader.read_line(&mut msg){
        //    if n > 0 {tx.send(msg).unwrap();}
        //}
        
        thread::sleep(Duration::from_millis(1000));
    });
}

fn send_all(clients:Vec<TcpStream>, s: &str) -> Vec<TcpStream>{
    let mut collector = vec![];
    for mut socket in clients.into_iter(){
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes){
            println!("Transfer error : {}",e);
            continue;
        }
        collector.push(socket);
    }
    collector
}