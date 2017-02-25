//Importing damn crates
extern crate thrussh;
extern crate futures;
extern crate tokio_core;
extern crate env_logger;
extern crate iron;
extern crate rustc_serialize;
use rustc_serialize::hex::{ToHex};
extern crate mount;
extern crate staticfile;
extern crate router;

use std::collections::HashMap;
use std::sync::Arc;
use std::io;
use std::path::Path;

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

    fn tcpip_forward(self, address: &str, port: u32, session: server::Session) -> Self::FutureBool
    {
        println!("Client {} requested a port forward to {}", address, port);
        futures::finished((self, session, true))
    }
}

fn runSSHServer() {
    env_logger::init().unwrap();
    // Starting the server thread.
    let t = std::thread::spawn(|| {
        let mut config = thrussh::server::Config::default();

        for (index, flag) in config.methods.enumerate() {
            if index != 4 {
                config.methods.remove(flag);
            }
        }

        println!("{:?}", config.methods);

        config.connection_timeout = Some(std::time::Duration::from_secs(600));
        config.auth_rejection_time = std::time::Duration::from_secs(3);
        config.keys.push(thrussh::key::Algorithm::generate_keypair(thrussh::key::ED25519).unwrap());

        let config = Arc::new(config);
        let sh = H{};
        thrussh::server::run(config, "0.0.0.0:2222", sh);
    });
}
fn respond(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn start_web() 
{
    //Creating the router
    let mut router = Router::new();
    router.get("/",respond,  "hello");

    let mut mount = Mount::new();
    mount .mount("/",Static::new(Path::new("src/static")));
    
    //Creating the server
    Iron::new(mount).http("localhost:8000").unwrap();
    
}

fn main() {
    //startWeb();
    runSSHServer();
    start_web();
    let mut x = String::new();
    io::stdin().read_line(&mut x);
}

