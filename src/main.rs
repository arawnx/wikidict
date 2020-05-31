extern crate wikipedia;

use std::env;
use std::collections::HashMap;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let mut num = 0;
    for argument in env::args() {
        if argument.parse::<i32>().is_ok() {
            num = argument.parse::<i32>().unwrap();
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
            for title in titles {
                let page = wiki.page_from_title(title);
                let summary = page.get_summary().unwrap();
                t.send(summary).unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut summaries = Vec::new();
    for rec in r.try_iter() {
        summaries.push(rec.to_string());
    }

    let summaries = summaries.join(" ");
    let words: Vec<String> = summaries.split_ascii_whitespace().map(|w| w.to_owned()).collect();
    let words: Vec<String> = words.into_iter().filter(|w| {
        let mut res = true;
        for c in w.chars() {
            if !c.is_ascii_alphabetic() {
                res = false;
            }
        }
        return res;
    }).collect();
    let mut words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();
    words.sort();
    words.dedup();
    words.sort_by(|a, b| {
        return a.len().partial_cmp(&b.len()).unwrap();
    });

    print!("[");
    let mut out = String::new();
    for w in words {
        out = format!("{}\"{}\",", out, w);
    }
    out.pop();
    print!("{}", out);
    print!("]");
}
