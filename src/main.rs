//Importing damn crates
extern crate thrussh;
extern crate futures;
extern crate tokio_core;
extern crate env_logger;
extern crate iron;
extern crate rustc_serialize;
extern crate params;
use rustc_serialize::hex::{ToHex};
extern crate mount;
extern crate staticfile;
extern crate router;

use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use std::option::Option;

use thrussh::*;
use thrussh::server::Response as sshResponse;

use iron::prelude::*;
use iron::status;
use iron::Handler;

use mount::Mount;
use router::Router;
use staticfile::Static;


#[derive(Clone)]
struct H{}

impl server::Handler for H {
    type Error = ();
    type FutureAuth = futures::Finished<(Self, server::Auth), Self::Error>;
    type FutureUnit = futures::Finished<(Self, server::Session), Self::Error>;
    type FutureBool = futures::Finished<(Self, server::Session, bool), Self::Error>;

    fn auth_keyboard_interactive(self,user:&str,submethods:&str, response:Option<sshResponse>)-> Self::FutureAuth{
        println!("KeyboardInteractiveLogin user: {:?}, response: {:?}, submethods: {:?}", user, response, submethods);
        futures::finished((self, server::Auth::Accept))
    }

    fn data(mut self, channel: ChannelId, data: &[u8], mut session: server::Session) -> Self::FutureUnit {
        println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
        session.data(channel, None, data).unwrap();
        futures::finished((self, session))
    }

    fn extended_data(self, channel: ChannelId, code: u32, data: &[u8], session: server::Session) -> Self::FutureUnit {
        println!("data on channel {:?}.{}: {:?}", channel, code, data.to_hex());
        futures::finished((self, session))
    }

    // When the SSH client requests a TCP/IP forward this method is called
    fn tcpip_forward(self, address: &str, port: u32, mut session: server::Session) -> Self::FutureBool
    {
        println!("Client {} requested a port forward to {}", address, port);
        let ch_id = session.channel_open_forwarded_tcpip(address, port, "localhost", port).unwrap();
        println!("New TCP/IP forwarding session on channel {:?}", ch_id);
        // Send a get request over the new channel
        session.request_success();
        session.data(ch_id, None, "GET / HTTP/1.1\r\n".as_bytes());
        
        // Return true that the tcpip forward request is accepted
        futures::finished((self, session, true))
    }

    fn channel_open_session(self, channel: ChannelId, session: server::Session) -> Self::FutureUnit
    {
        println!("New session {:?}", channel);
        futures::finished((self, session))
    }

	fn channel_open_direct_tcpip(self, channel: ChannelId,
		 host_to_connect: &str, port_to_connect: u32,
		 originator_address: &str, originator_port: u32,
		 session: server::Session) -> Self::FutureUnit
    {
        println!("Channel open direct tcpip: {} {} {} {}",host_to_connect,
                 port_to_connect,originator_address,originator_port);

        futures::finished((self, session))
    }
}

fn run_ssh_server() {
    env_logger::init().unwrap();
    // Starting the server thread.
    std::thread::spawn(|| {
        let mut config = thrussh::server::Config::default();

        for (index, flag) in config.methods.enumerate() {
            if index != 4 {
                config.methods.remove(flag);
            }
        }

        config.connection_timeout = Some(std::time::Duration::from_secs(600));
        config.auth_rejection_time = std::time::Duration::from_secs(3);
        config.keys.push(thrussh::key::Algorithm::generate_keypair(thrussh::key::ED25519).unwrap());

        let config = Arc::new(config);
        let sh = H{};
        thrussh::server::run(config, "0.0.0.0:2222", sh);
    });
}
fn portReceive(req: &mut Request) -> IronResult<Response> {
    println!("{:?}",req.method);
    let url = req.url.path();
    let ref slug = url[0]; 
    let mut location = std::string::String::new();
    let mut cur = url[0];
    for i in 1..url.len()
    {
        cur = url[i];
        println!("{}",i);
        location.push_str("/");
        location.push_str(&cur);
    }
    let msg = "Slug= ".to_string() + slug + " location= " + &location;
    println!("Req is = {:?}", location);
    Ok(Response::with((status::Ok, msg)))
}

fn start_web() 
{
    //Creating the router
    let mut router = Router::new();
    router.get("*",portReceive,  "hello");
    router.post("*",portReceive,  "hello");
    router.put("*",portReceive,  "hello");

    let mut mount = Mount::new();
    mount.mount("/s/", router)
        .mount("/",Static::new(Path::new("src/static")));

    //Creating the server
    Iron::new(mount).http("localhost:8000").unwrap();
}

fn main() {
    println!(
        " _____       _      _                            \n\
        |  _  |     (_)    | |                           \n\
        | | | |_   _ _  ___| | _____  ___ _ ____   _____ \n\
        | | | | | | | |/ __| |/ / __|/ _ \\ '__\\ \\ / / _ \\\n\
        \\ \\/' / |_| | | (__|   <\\__ \\  __/ |   \\ V /  __/\n \
         \\_/\\_\\\\__,_|_|\\___|_|\\_\\___/\\___|_|    \\_/ \\___|");

    run_ssh_server();
    start_web();
}

