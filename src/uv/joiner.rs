extern crate rand;

use self::rand::random;
use std::{os, env};
use std::fs::File;
use std::path::Path;
use std::boxed::Box;
use std::io::{Read, Write};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]); 
    } else {
        let fname = &args[1];
        let path = Path::new(&fname);
        let msg_file = File::open(&path);

        match (msg_file) {
            Ok(mut msg) => {
                let mut msg_bytes = vec![];
                Read::read_to_end(&mut msg, &mut msg_bytes);
                let share1_file 
                       = File::create(Path::new(&(fname.clone() + ".share1")));
                let share2_file 
                       = File::create(Path::new(&(fname.clone() + ".share2")));
                
                match (share1_file, share2_file) {
                    (Ok(mut share1), Ok(mut share2)) => { 
                        let cipher = split(msg_bytes.as_slice());
                        share1.write(cipher.0.as_slice());
                        share2.write(cipher.1.as_slice());
                    } ,
                    (_, _) => panic!("Error opening output files!"),
                }
            } ,
            Err(_) => panic!("Error opening message file: {}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(a.len());
    for i in 0 .. a.len() {
	    ret.push(a[i] ^ b[i]);
    }
    ret
}

pub fn split(msg_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut random_bytes = Vec::with_capacity(msg_bytes.len());
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in 0 .. msg_bytes.len() {
        let random_byte = random();
        random_bytes.push(random_byte);
    }
    
    let encrypted_bytes = xor(msg_bytes, random_bytes.as_slice());
    
    (random_bytes, encrypted_bytes)
}

// Solution to joiner exercise
pub fn join(enc_bytes: &[u8], rnd_bytes: &[u8]) -> Vec<u8> {
    assert_eq!(enc_bytes.len(), rnd_bytes.len());
    xor(enc_bytes, rnd_bytes)
}