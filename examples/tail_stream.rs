extern crate gerrit_stream;
use failure;

fn main() -> Result<(), failure::Error> {
    let filters: Vec<&str> = vec!["change-merged"];
    let channel = gerrit_stream::init("Chris.MacNaughton", "review.openstack.org", 29418, filters)?;
    for message in channel.iter() {
        match message {
            gerrit_stream::GerritMessage::ChangeMerged(change_merged) => {
                println!("Change merged: {}", change_merged.change.url);
            }
            other => println!("Something weird slipped through!\n{:?}", other),
        }
        // if message["type"] == "change-merged" {
        //     println!("\nLanded: {:?}\n", message);
        // } else {
        // println!("\n{:?}\n", message);
        // }
    }
    Ok(())
}
