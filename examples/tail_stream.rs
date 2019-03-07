extern crate gerrit_stream;

fn main() -> Result<(), String> {
    let channel = gerrit_stream::init("Chris.MacNaughton", "review.openstack.org", 29418)?;
    for message in channel.iter() {
        if message["type"] == "change-merged" {
            println!("\nLanded: {:?}\n", message);
        } else {
            println!("\n{}: {:?}\n", message["type"], message);
        }
    }
    Ok(())
}