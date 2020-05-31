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
<<<<<<< HEAD
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
=======
    for _ in 0..num {
        let mut titles = Vec::new();
        match wiki.random_count(1) {
            Result::Err(msg) => {
                eprintln!("{}", msg);
            },
            Ok(x) =>  {
                titles = x;
            }
        }
>>>>>>> 39e7ad6813f224c6a085b04116bb198e029f1482

    let wordsum = Arc::new(Mutex::new(Vec::new()));

<<<<<<< HEAD
    let mut handles = Vec::new();
    for sum in sums {
        let local = Arc::clone(&wordsum);
        let handle = thread::spawn(move || {
            let words: Vec<String> = sum.split_ascii_whitespace().map(|w| w.to_owned()).collect();
            let words: Vec<String> = words.into_iter().filter(|w| w.chars().all(char::is_alphabetic)).collect();
            let mut words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();
            local.lock().unwrap().append(&mut words);
        });
=======
            let words: Vec<String> = contents.split_ascii_whitespace().map(|w| w.to_owned()).collect();
            let words: Vec<String> = words.into_iter().filter(|w| {
                for c in w.chars() {
                    if !c.is_ascii_alphabetic() {
                        return false;
                    }
                }

                return true;
            }).collect();
            let words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();
>>>>>>> 39e7ad6813f224c6a085b04116bb198e029f1482

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
        print!("{{");
        let mut conts = String::new();
        for (k, v) in dictionary {
            conts = format!("{}\n\"{}\": {},", conts, k, v);
        }
        conts.pop();
        print!("{}", conts);
        print!("\n}}");
    } else {
        print!("[");
        let mut conts = String::new();
        for key in dictionary.keys() {
            conts = format!("{}\n\"{}\",", conts, key);
        }
        conts.pop();
        print!("{}", conts);
        print!("\n]");
    }
    */
}
