use std::net::{TcpListener, TcpStream};

fn main() 
{
    #![allow(non_snake_case)]
    println!("Hello, world!");
    start_web();
}

fn start_web() 
{
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    //accept connections and processing them
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) => handle_client(stream),
            Err(_) => println!("Connection failed")
        }
    }

}
fn handle_client(stream:TcpStream) 
{
    println!("Connection handled");
    //
}
