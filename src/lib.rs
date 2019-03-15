use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[macro_use]
extern crate serde_derive;
use failure;
use ssh2::Session;
use std::io::BufReader;

pub mod gerrit_message;

pub use gerrit_message::GerritMessage;

pub fn with_channel<T1, T2, T3, T4, Ref1>(
    tx: Sender<GerritMessage>,
    name: T1,
    url: T2,
    port: u16,
    ssh_key: T3,
    password: Option<T4>,
    filters: Vec<Ref1>,
) -> Result<(), failure::Error>
where
    T1: Into<String>,
    T2: Into<String>,
    T3: Into<String>,
    T4: Into<String>,
    Ref1: AsRef<str>,
{
    let name = name.into();
    let url = url.into();
    let ssh_key = ssh_key.into();
    let password = password.map(|a| a.into());
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
        match password {
            Some(password) => sess
                .userauth_pubkey_memory(name.as_ref(), None, ssh_key.as_ref(), Some(&password))
                .expect("Can't authenticate"),
            None => sess
                .userauth_pubkey_memory(name.as_ref(), None, ssh_key.as_ref(), None)
                .expect("Can't authenticate"),
        }
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

pub fn init<T1, T2, T3, T4, Ref1>(
    name: T1,
    url: T2,
    port: u16,
    ssh_key: T3,
    password: Option<T4>,
    filters: Vec<Ref1>,
) -> Result<(Receiver<GerritMessage>), failure::Error>
where
    T1: Into<String>,
    T2: Into<String>,
    T3: Into<String>,
    T4: Into<String>,
    Ref1: AsRef<str>,
{
    let (tx, rx) = channel();
    with_channel(tx, name, url, port, ssh_key, password, filters)?;
    Ok(rx)
}
