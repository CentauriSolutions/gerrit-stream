use std::thread;
use std::sync::mpsc::{Receiver, channel};
use std::io::prelude::*;
use std::net::{TcpStream};

use serde::{Deserialize, Serialize};
#[macro_use]
extern crate serde_derive;
use std::io::BufReader;
use ssh2::Session;

// use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// pub struct GerritMessage;

pub fn init<T1: Into<String>, T2: Into<String>>(name: T1, url: T2, port: u16) -> Result<Receiver<serde_json::Value>, String> {
    let (tx, rx) = channel();
    let name = name.into();
    let url = url.into();
    // let filter = filter.into();

    thread::spawn(move|| {
        let tcp = TcpStream::connect(format!("{}:{}", url, port)).expect("Can't create a TCP Socket");
        let mut sess = Session::new().expect("Can't  create a SSH session");
        sess.handshake(&tcp).expect("Can't connect the SSH session to the TCP Socket");

        // Try to authenticate with the first identity in the agent.
        sess.userauth_agent(name.as_ref()).expect("Can't authenticate");
        let mut channel = sess.channel_session().expect("Couldn't create SSH channel");

        channel.exec("gerrit stream-events").expect("couldn't start stream");
        let reader = BufReader::new(channel);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(value) = serde_json::from_str(&line) {
                    match tx.send(value) {
                        Err(e) => println!("{:?}", e),
                        _ => {},
                    }
                } else {
                    println!("Failed to JSON parse Line: {}", line);
                }
            }
        }
    });

    Ok(rx)
}