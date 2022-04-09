use std::net::{TcpListener, TcpStream, Shutdown};
use std::collections::HashMap;
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;
use std::io::{Read, Write};

const IP_ADDRESS: &str = "127.0.0.1:3000";

fn main() {
  let listener = TcpListener::bind(IP_ADDRESS).unwrap();

  let map: HashMap<String, TcpStream> = HashMap::new();
  let clients = Arc::new(Mutex::new(map));

  println!("Server listening on port 3000");

  for stream in listener.incoming() {
    let clone_clients = Arc::clone(&clients);

    match stream {
      Ok(stream) => {
        thread::spawn(move || {
          handle(stream, clone_clients);
        });
      }
      Err(e) => panic!("error: {}", e),
    }
  }
}

fn handle(mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
  let mut data = [0 as u8; 100];
  while match stream.read(&mut data) {
    Ok(size) => {
      let lock_clients = clients.lock().unwrap();
      let message = String::from_utf8_lossy(&data[0..size]).to_string();
      let parsed = parse_data(message, &stream, lock_clients);
      if parsed == "exit" {
        return
      }
      let clone_clients = clients.clone();
      let lock_clone_clients = clone_clients.lock().unwrap();
      let values = lock_clone_clients.values();

      for mut stream in values {
        stream.write(parsed.as_bytes()).expect("Failed to reply");
      }

      true
    },
    Err(_) => {
      stream.shutdown(Shutdown::Both).unwrap();
      return
    }
  } {}
}

fn parse_data(data: String, mut stream: &TcpStream, mut users: MutexGuard<HashMap<String, TcpStream>>) -> String {
  let args: Vec<&str> = data.split("\n").collect();
  let clone_stream = stream.try_clone().expect("Failed to clone!");

  if args[0] == "login" {
    match users.get(args[1]) {
      Some(_) => {
        let data = format!("name duplicated!");
        stream.write(data.as_bytes()).expect("Failed to reply duplicated name");
        return String::from("");
      }
      None => {
        users.insert(args[1].to_string(), clone_stream);                         
        return format!("{} is login to server!", args[1])
      },
    };
  }
  println!("1");
  let username = args[0].to_string();
  let message = args[1].to_string();

  if &message[0..1] != "/" {
    return format!("{}: {}", username, message)
  }
  
  handle_command(stream, message[1..].to_string(), username, users)
}

fn handle_command(mut stream: &TcpStream, message: String, username: String, mut users: MutexGuard<HashMap<String, TcpStream>>) -> String{
  let args: Vec<&str> = message.split(" ").collect::<Vec<&str>>();
  let command = args[0];
  match command {
    "exit" | "e" => {
      users.remove(&username);

      return String::from("exit")
    },

    "list" | "l" => {
      let usernames = users.keys();

      let mut list = String::with_capacity(users.len());
      let mut length = 0;
      let mut i = 0;

      for name in usernames {
        if i == users.len() - 1 {
          list.insert_str(length, name); 
        } else {
          let format = format!("{}\n", name);
          list.insert_str(length, format.as_str());
          length += format.len();
          i += 1;
        }
      }

      stream.write(list.as_bytes()).expect("Failed to reply");
      return String::from("");
    },

    "whisper" | "w" => {
      let mut user_stream = stream;
      let mut target_stream = users.get(args[1]).unwrap();
      let message = args[2..].join(" ");

      user_stream.write(message.as_bytes()).expect("Failed to whisper!");
      target_stream.write(message.as_bytes()).expect("Failed to whisper!");

      return String::from("");
    },
    &_ => { 
      let data = String::from("unknown command");
      return data
    }
  }
  
}