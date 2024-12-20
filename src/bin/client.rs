use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:8000")
        .expect("connect function failed");
    let buf = vec![1, 1, 1, 1];
    let _ = socket.send(&buf);
}
