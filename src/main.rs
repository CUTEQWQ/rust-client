//extern crate time;
extern crate package_handler;

use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;
use time::*;
use std::cell::RefCell;
use std::fmt;
use package_handler::*;
use std::{thread,time};
use std::sync::{Arc,Mutex};
//use std::time::Duration;

fn main(){
    //serial download test
     //let mut result=Vec::new();

     //let start=time::get_time();
     let mut f=File::create("gfss.pdf").unwrap();
     let mut stream=TcpStream::connect("0.0.0.0:6060").unwrap();

    let mut newstream = mystream::new(stream);
    //let mut newstream = stream_without_lock::new(stream);

    //写线程
    let mut newstream2 = newstream.clone();
    let writeth = thread::spawn(move || {
        println!("in write");
        for _ in 0..2 {
            let request = String::from("download");
            let p = create_request(request);
            println!("{:?}", p);
            let size1 = newstream2.astream.lock().unwrap().write(p.as_slice()).unwrap();
            println!("send ok! size1 :{:?}", size1);
        }
        let request = String::from("check");
        let p = create_request(request);
        println!("{:?}", p);
        let size1 = newstream2.astream.lock().unwrap().write(p.as_slice()).unwrap();
        println!("send ok! size1 :{:?}", size1);
        //thread::sleep(time::Duration::from_millis(10));
    });
    //writeth.join().unwrap();

    let mut newstream3 = newstream.clone();
    let readth = thread::spawn(move || {
        println!("in read");
        loop {
            //println!("in read loop");
            {
                let mut a = newstream3.astream.lock().unwrap();
                let head = read_certain_bytes(&a ,4);
                if head.len() == 0 {
                    continue;
                }
                let size = download_file(&f,&a,head);
                println!("finish load file, totally {:?} bytes",size);
            }
            //thread::sleep(time::Duration::from_millis(10));
        }
    });

    //writeth.join().unwrap();
//    readth.join().unwrap();
loop {}

          //let size = download_file(&f, &stream);
          //println!("size is {:?}",size);
          //let end=time::get_time();
          //println!("finish the request cost: {} ",end-start);
          //result.push(end-start);

     //let sum=result.iter().fold(Duration::nanoseconds(0),|acc,&x|acc+x);
     //let result=sum/3;

     //println!("Total time is {} ,Average time is {}",sum, result);


}