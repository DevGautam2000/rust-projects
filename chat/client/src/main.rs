use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const PORT: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {

    //try catch is handles with panics!
    let mut client = TcpStream::connect(PORT).expect("Stream failed to connect"); //create a connection
    client.set_nonblocking(true).expect("failed to initiate non-blocking"); //intiate net blocking

    let (transmitter, receiver) = mpsc::channel::<String>(); //structure bind the channel varaibles

    thread::spawn(move || loop { //spawn the thread and create a indefinite loop
        let mut buff = vec![0; MSG_SIZE]; //initiate the buffer
        match client.read_exact(&mut buff) {
            Ok(_) => { //if status ok
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); //keep reading from buffer
                println!("message recv {:?}", msg); //print message 
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), //if erroe of kind WouldBlock
            Err(_) => { //catch
                println!("connection with server was severed");
                break; //break if error from loop
            }
        }

        match receiver.try_recv() { //try receive
            Ok(msg) => { // if status ok
                let mut buff = msg.clone().into_bytes(); //cline the message into bytes
                buff.resize(MSG_SIZE, 0); //resize the buffer
                client.write_all(&buff).expect("writing to socket failed"); //write the message to the socket
                println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (), //if error empty or
            Err(TryRecvError::Disconnected) => break //if error disconnected break
        }

        sleep(); // make the thread sleep for 100ms as in a loop
    });

    //read message from user
    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || transmitter.send(msg).is_err() {break}
    }
}