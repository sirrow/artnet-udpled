use artnet_protocol::*;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

fn main() {
    let ledsocket = UdpSocket::bind("0.0.0.0:9999").expect("failed to bind socket");
    ledsocket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    ledsocket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();


    let socket = UdpSocket::bind(("127.0.0.1", 6454)).unwrap();
    let broadcast_addr = ("255.255.255.255", 6454)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    socket.set_broadcast(true).unwrap();
    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();

    let mut zerobuf :[u8; 510] = [0u8; 510];
    let mut onebuf :[u8; 258] = [0u8; 258];
    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();

        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        match command {
            ArtCommand::Poll(poll) => {
                // This will most likely be our own poll request, as this is broadcast to all devices on the network
            }
            ArtCommand::PollReply(reply) => {
                // This is an ArtNet node on the network. We can send commands to it like this:
                /*
                let command = ArtCommand::Output(Output {
                    length: 5,                 // must match your data.len()
                    data: vec![1, 2, 3, 4, 5], // The data we're sending to the node
                    ..Output::default()
                });
                let bytes = command.write_to_buffer().unwrap();
                socket.send_to(&bytes, &addr).unwrap();
                */
            }
            ArtCommand::Output(output) => {
                /*
                let version = output.version;
                println!("version {:?}", version);
                let length = output.length;
                println!("len {:?}", length);
                */
                if output.physical == 0 {
                    zerobuf.copy_from_slice(output.data.inner.as_slice());
                }
                if output.physical == 1 {
                    onebuf.copy_from_slice(output.data.inner.as_slice());
                }

                let mut zerovec = zerobuf.to_vec();
                let mut onevec = onebuf.to_vec();
                zerovec.append(&mut onevec);
                let sendbuffer = zerovec.as_slice();

                ledsocket.send_to(&sendbuffer, "192.168.2.51:8888").expect("failed to send data");
            }
            _ => {}
        }
    }

}
