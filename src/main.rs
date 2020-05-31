extern crate wikipedia;

use std::env;
use std::collections::HashMap;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let mut num = 0;
    let mut use_frequency = true;
    for argument in env::args() {
        if argument.parse::<i32>().is_ok() {
            num = argument.parse::<i32>().unwrap();
        }

        if argument == "-a" { // Output as array without frequency if -a is applied
            use_frequency = false;
        }
    }

    let wiki = Arc::new(Mutex::new(wikipedia::Wikipedia::<wikipedia::http::default::Client>::default()));
    let (t, r) = mpsc::channel();

    let mut handles = Vec::new();
    for _ in 0..num {
        let wiki = Arc::clone(&wiki);
        let t = mpsc::Sender::clone(&t);
        let handle = thread::spawn(move || {
            let wiki = wiki.lock().unwrap();
            let titles = wiki.random_count(255).unwrap();
            let page = wiki.page_from_title(title);
            let summary = page.get_summary().unwrap();
            t.send(summary).unwrap();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for rec in r.try_iter() {
        println!("{}", rec);
    }
}
