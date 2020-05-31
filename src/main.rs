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


    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let mut titles = Vec::new();
    for _ in 0..num {
        titles.push(wiki.random().unwrap().unwrap());
    }
    let mut pages = Vec::new();
    for title in titles {
        pages.push(wiki.page_from_title(title));
    }
    let mut sums = Vec::new();
    for page in pages {
        sums.push(page.get_summary().unwrap());
    }

    let wordsum = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for sum in sums {
        let local = Arc::clone(&wordsum);
        let handle = thread::spawn(move || {
            let words: Vec<String> = sum.split_ascii_whitespace().map(|w| w.to_owned()).collect();
            let words: Vec<String> = words.into_iter().filter(|w| w.chars().all(char::is_alphabetic)).collect();
            let mut words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();
            local.lock().unwrap().append(&mut words);
        });

        handles.push(handle);

        /*for word in &words {
            if dictionary.contains_key(word) {
                *dictionary.get_mut(word).unwrap() += 1;
            } else {
                dictionary.insert(word.to_owned(), 1);
            }
        }*/
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *wordsum.lock().unwrap());

    /*
    if use_frequency {
        println!("{:?}", dictionary);
    } else {
        println!("{{[");
        for key in dictionary.keys() {
            println!("{:?},", key);
        }
        println!("]}}");
    }
    */
}
