extern crate gerrit_stream;

fn main() -> Result<(), String> {
    let channel = gerrit_stream::init("Chris.MacNaughton", "review.openstack.org", "comment-added", 29418)?;
    for message in channel.iter() {
        println!("Recieved: {:?}", message);
    }
    Ok(())
}