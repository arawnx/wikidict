extern crate wikipedia;

use std::env;
use std::collections::HashMap;

fn main() {
    let mut num = 0;
    for argument in env::args() {
        if argument.parse::<i32>().is_ok() {
            num = argument.parse::<i32>().unwrap();
        }
    }

    let mut dictionary: HashMap<String, i32> = HashMap::new();

    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    for _ in 0..num {
        let mut titles = Vec::new();
        match wiki.random_count(255) {
            Result::Err(msg) => {
                eprintln!("{}", msg);
            },
            Ok(x) =>  {
                titles = x;
            }
        }

        let mut contents = String::new();
        for title in titles {
            match wiki.page_from_title(title).get_summary() {
                Ok(x) => {
                    contents = x;
                },
                Err(msg) => {
                    eprintln!("{}", msg);
                }
            }

            let words: Vec<String> = contents.split_ascii_whitespace().map(|w| w.to_owned()).collect();
            let words: Vec<String> = words.into_iter().filter(|w| w.chars().all(char::is_alphanumeric)).collect();
            let words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();

            for word in &words {
                if dictionary.contains_key(word) {
                    *dictionary.get_mut(word).unwrap() += 1;
                } else {
                    dictionary.insert(word.to_owned(), 1);
                }
            }
        }
    }

    println!("{:?}", dictionary);
}
