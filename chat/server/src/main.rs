use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const PORT: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(PORT).expect("Listener failed to bind"); //bind the listerener to server
    server.set_nonblocking(true).expect("failed to initialize non-blocking"); //initialize net_blocking

    let mut clients = vec![];//to create multiple clients
    let (transmitter, receiver) = mpsc::channel::<String>(); //transmitter and receiver
    loop {
        if let Ok((mut socket, addr)) = server.accept() { //accept if status ok
            println!("Client {} connected", addr);

            let transmitter = transmitter.clone(); //clone the transmitter to pass in thread
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop { //spawn thread and loop
                let mut buff = vec![0; MSG_SIZE]; //buffer

                match socket.read_exact(&mut buff) { 
                    Ok(_) => { //if status ok
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); //read from buffer
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        transmitter.send(msg).expect("failed to send msg to receiver"); //send message to receiver
                    }, 
                    // if error kind wouldblock break loop
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                sleep(); //sleep thread for every 100ms
            });
        }

       
        if let Ok(msg) = receiver.try_recv() {  //if messge received
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes(); //clone message 
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()// write! all message to vector[]
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}