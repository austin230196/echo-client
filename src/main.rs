use std::{process, net::TcpStream, env, io::prelude::*, io};

mod lib;
use lib::*;




fn main(){
    //load env files
    helpers::get_env();
    //first connect to the server
    let server = env::var("ECHO_HOST").unwrap() + ":" + env::var("ECHO_PORT").unwrap().as_str();
    println!("Connecting to {}", server);

    //then connect to the tcp stream
    let mut stream = TcpStream::connect(server).unwrap_or_else(|err| {
        println!("#main: {}", err);
        process::exit(1);
    });
    println!("Connected to server IP Address: {}, Port: {}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
    let mut repeat = String::new();
    io::stdin().read_line(&mut repeat).unwrap_or_else(|err| {
        println!("#main: {}", err);
        process::exit(2);
    });

    //sent message
    let _ = stream.write(repeat.as_bytes());
    let _ = stream.flush();


    //read message from stream;
    let mut empty_buffer = Vec::from([0;1024]);
    let _ = stream.read(&mut empty_buffer);
    // empty_buffer = empty_buffer.into_iter().filter(|n| n > &0).collect::<Vec<u8>>();
    let response = String::from_utf8_lossy(&empty_buffer).trim().to_owned();
    let _ = io::stdout().write(response.as_bytes());
    while repeat != "quit" {
        repeat = String::new();
        io::stdin().read_line(&mut repeat).unwrap_or_else(|err| {
            println!("#main: {}", err);
            process::exit(2);
        });
    
        //sent message
        let _ =stream.write(repeat.as_bytes());
        let _ = stream.flush();
    
    
        //read message from stream;
        let mut empty_buffer = Vec::from([0;1024]);
        let _ = stream.read(&mut empty_buffer);
        // empty_buffer = empty_buffer.into_iter().filter(|n| n > &0).collect::<Vec<u8>>();
        let response = String::from_utf8_lossy(&empty_buffer).trim().to_owned();
        let _ = io::stdout().write(response.as_bytes());
    }

}