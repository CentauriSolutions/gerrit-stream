use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[macro_use]
extern crate serde_derive;
use failure;
use ssh2::Session;
use std::io::BufReader;

mod gerrit_message;

pub use gerrit_message::GerritMessage;

pub fn with_channel<T1: Into<String>, T2: Into<String>, T3: AsRef<str>>(
    tx: Sender<GerritMessage>,
    name: T1,
    url: T2,
    port: u16,
    filters: Vec<T3>,
) -> Result<(), failure::Error> {
    let name = name.into();
    let url = url.into();
    let filter_str: String = filters
        .iter()
        .map(|f| f.as_ref())
        .collect::<Vec<&str>>()
        .join(" -s ");

    thread::spawn(move || {
        let tcp =
            TcpStream::connect(format!("{}:{}", url, port)).expect("Can't create a TCP Socket");
        let mut sess = Session::new().expect("Can't  create a SSH session");
        sess.handshake(&tcp)
            .expect("Can't connect the SSH session to the TCP Socket");

        // Try to authenticate with the first identity in the agent.
        sess.userauth_agent(name.as_ref())
            .expect("Can't authenticate");
        let mut channel = sess.channel_session().expect("Couldn't create SSH channel");
        let mut exec_str = "gerrit stream-events".into();
        if filter_str != "" {
            exec_str = format!("{} -s {}", exec_str, filter_str);
        }
        channel.exec(&exec_str).expect("couldn't start stream");
        let reader = BufReader::new(channel);
        for line in reader.lines() {
            if let Ok(line) = line {
                match serde_json::from_str(&line) {
                    Ok(message) => {
                        if let Err(e) = tx.send(message) {
                            println!("{:?}", e)
                        }
                    }
                    Err(e) => {
                        println!("\n\tFailed to parse JSON:{:?}\n{}", e, line);
                    }
                }
            }
        }
    });
    Ok(())
}

pub fn init<T1: Into<String>, T2: Into<String>, T3: AsRef<str>>(
    name: T1,
    url: T2,
    port: u16,
    filters: Vec<T3>,
) -> Result<Receiver<GerritMessage>, failure::Error> {
    let (tx, rx) = channel();
    with_channel(tx, name, url, port, filters)?;
    Ok(rx)
}
