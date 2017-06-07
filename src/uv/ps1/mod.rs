//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans and Muhammad Nael
// Version 0.4

#![allow(non_snake_case)]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::collections::HashSet;
use std::sync::Mutex;
use std::ops::Index;

use std::convert::{Into, From};

use super::utils::ascii_string::{ StringExt };
use super::utils::*;

static mut n_requests: usize = 0;

// lazy_static!{
//     static ref PREV_IPS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
// }


enum RequestType {
    Get,
    Post,
}

struct QueryArgs {}

struct Query {
    api: String,
    args: Either<String, QueryArgs>
}

pub struct HttpRequest {
    req_type: RequestType,
    query: Query,
    body: String,
}

impl HttpRequest {
}


pub fn zhttpto() {
    let addr = "127.0.0.1:4414";

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on [{}] ...", addr);


    for stream in listener.incoming() {
        match stream {
            Err(_) => (),
            Ok(mut stream) => {
                // Spawn a thread to handle the connection
                thread::spawn(move|| {
                    let mut visitor = unsafe { n_requests };

                    match stream.peer_addr() {
                        Err(_) => {}
                        Ok(pn) => {
                            {
                                // let peer_addr_str = format!("{}", pn.ip());
                                // let mut set = PREV_IPS.lock().unwrap();
                                // if !set.contains(&peer_addr_str) {
                                //     unsafe {
                                //         visitor = n_requests + 1;
                                //         n_requests = visitor;
                                //     }
                                //     set.insert(peer_addr_str);
                                //     println!("Received NEW connection from: [{}]", pn);
                                // }
                                // else {
                                //     println!("Received connection from: [{}]", pn);
                                // }
                            }
                            unsafe {
                                visitor = n_requests + 1;
                                n_requests = visitor;
                            }
                            println!("Received connection from: [{}]", pn);
                        }
                    }

                    let mut request = "".to_string();
                    let mut buf = [0 ; 500];
                    stream.read(&mut buf).unwrap();
                    match str::from_utf8(&buf) {
                        Err(error) => println!("Received request error:\n{}", error),
                        Ok(body) => {
                            println!("Recieved request body:\n{}", body);
                            request = body.to_string();
                        }
                    }

                    // let request = HttpRequest::new(request);

                    // let call = request.api_call();

                    let response =
                        format!("{}", visitor);
                    stream.write(response.as_bytes()).unwrap();
                    println!("Connection terminates.");
                });
            },
        }
    }

    drop(listener);
}