extern crate iron;
#[allow(unused_imports)]
use iron::prelude::*;
#[allow(unused_imports)]
use iron::status;

fn main() 
{
    #![allow(non_snake_case)]
    println!("Starting server");
    start_web();
}

fn start_web() 
{
    Iron::new(|_:&mut Request|
              {
                  Ok(Response::with((status::Ok, "Hello!\r\n")))
              }).http("localhost:8000").unwrap();
    
}
