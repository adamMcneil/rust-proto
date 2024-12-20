use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:8000")?;
        match socket.local_addr() {
            Ok(socket_addr) => {
                println!("{:?}", socket_addr);
            }
            Err(_) => todo!(),
        }
        let mut buf = [0; 100];
        loop {
            let (_amt, _src) = socket.recv_from(&mut buf)?;
            let result = str::from_utf8(&buf[..100])
                .map(String::from)
                .expect("Buffer contains invalid UTF-8");
            println!("{:?}", result);
        }
    } // the socket is closed here
}
