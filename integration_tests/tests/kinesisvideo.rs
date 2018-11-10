#![cfg(feature = "kinesisvideo")]

extern crate rusoto_core;
extern crate rusoto_kinesisvideo;

use rusoto_kinesisvideo::{KinesisVideo, KinesisVideoClient, ListStreamsInput};
use rusoto_core::Region;

#[test]
fn should_list_streams() {
    let client = KinesisVideoClient::new(Region::UsEast1);
    let request = ListStreamsInput::default();

    let resp = client.list_streams(request).sync().unwrap();
    println!("Response is {:?}", resp);
}
