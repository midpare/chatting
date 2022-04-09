use std::net::{TcpStream};
use std::io::{Write, Read, stdin};
// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
use std::thread;

const IP_ADDRESS: &str = "127.0.0.1:3000";

fn main() {
  let mut stream = TcpStream::connect(IP_ADDRESS).expect("Failed to connect");
  let mut username = String::new();
  // let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

  let clone_stream = stream.try_clone().expect("Failed to clone");
  thread::spawn(move|| {
    handle(clone_stream);
  });

  loop {
    println!("Enter your name");
  
    stdin().read_line(&mut username).expect("Failed to read line");    
    username = username[0..username.len() - 2].to_string();
    
    let data = format!("login\n{}", username);
    stream.write(&data.as_bytes()).expect("Failed to transmit"); 
    // // let received = rx.recv().unwrap();
    // if received == "name duplicated" {
    //   return println!("name duplicated!");
    // }
    // println!("1");
    break
    
  }

  loop {
    // println!("1");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    let message = &input.as_bytes()[0..input.as_bytes().len() - 2];
    
    
    let data = format!("{}\n{}", username, String::from_utf8_lossy(&message));
    
    stream.write(&data.as_bytes()).expect("Failed to transmit");

    drop(data); 
    drop(message);
    
    if message == b"/exit" || message == b"/e" {
      break
    }
  };
}

fn handle(mut stream: TcpStream){
  let mut data = [0 as u8; 100];

  while match stream.read(&mut data) {
    Ok(size) => {
      // println!("1");
      let message = String::from_utf8_lossy(&data[0..size]);

      // if message == "name duplicated!" {
      //   return tx.send("name duplicated".to_string()).unwrap();
      // }
      println!("{}", message);
      true  
    },
    Err(e) => {
      println!("Error: {}", e);
      false
    }
  } {}
}
