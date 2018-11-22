#[macro_use]
extern crate futures;
extern crate tokio;

use std::time::{Duration, Instant};
use std::thread::sleep;

use futures::{Future, Stream, Poll, Async};
use std::fmt;

pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            curr: 1,
            next: 1,
        }
    }
}

impl Stream for Fibonacci {
    // type Item = ();
    type Item = u64;

    // The stream will never yield an error
    type Error = ();

    // fn poll(&mut self) -> Poll<Option<()>, Self::Error> {
    fn poll(&mut self) -> Poll<Option<u64>, ()> {
        println!("stream poll : {} , {}",self.curr,self.next);
        let curr = self.curr;
        let next = curr + self.next;
        
        self.curr = self.next;
        self.next = next;
        
        // let now = Instant::now();
        // sleep(Duration::new(3, 0));
        println!("1-stream poll : {} , {}",self.curr,self.next);

        Ok(Async::Ready(Some(curr)))
        // Ok(Async::Ready(Some(())))    
    }
}

pub struct Display10<T> {
    stream: T,
    curr: usize,
}

impl<T> Display10<T> {
    fn new(stream: T) -> Display10<T> {
        Display10 {
            stream,
            curr: 0,
        }
    }
}

impl<T> Future for Display10<T>
where
    T: Stream,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        println!("future poll");
        while self.curr < 20 {
            println!("while---future poll");
            let value = match try_ready!(self.stream.poll()) {
                Some(value) => value,
                // There were less than 10 values to display, terminate the
                // future.
                None => break,
            };

            println!("value #{} = {}", self.curr, value);
            self.curr += 1;
        }

        Ok(Async::Ready(()))
    }
}

fn main(){
    let fib = Fibonacci::new();
    // tokio::run(fib.map_err(|_| ()).for_each(|_| Ok(())));
    let display = Display10::new(fib);
    tokio::run(display);
}