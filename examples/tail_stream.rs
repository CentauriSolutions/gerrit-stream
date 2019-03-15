extern crate gerrit_stream;
use failure;
use std::env;
use std::fs::read_to_string;

fn main() -> Result<(), failure::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let Some(path) = args.next() {
        let password = args.next();
        let filters: Vec<&str> = vec!["change-merged"];
        let key = read_to_string(path)?;
        let channel = gerrit_stream::init(
            "gerrit-depender",
            "review.openstack.org",
            29418,
            key,
            password,
            filters,
        )?;
        for message in channel.iter() {
            match message {
                gerrit_stream::GerritMessage::ChangeMerged(change_merged) => {
                    println!("Change merged: {}", change_merged.change.url);
                }
                other => println!("Something weird slipped through!\n{:?}", other),
            }
        }
    } else {
        println!("Usage: tail_stream path/to/private_key");
    }
    Ok(())
}
