// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub fn create_join_request(name: String) -> messages::JoinReq {
    let mut request = messages::JoinReq::default();
    request.name = name;
    request
}
