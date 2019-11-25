#[macro_use]
extern crate clap;
use std::fs;
use std::io;
use std::io::{BufRead};
use std::collections::HashMap;
use rand::prelude::*;
use std::path;
use clap::App;
use fs::File;
use fs::OpenOptions;
use io::Write;
use path::Path;

// TODO: command line arguments

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let count: usize = matches.value_of("COUNT")
            .unwrap()
            .parse::<usize>()
            .expect("Could not parse <COUNT> value!");

        let ngram_size: usize = matches.value_of("n")
            .unwrap()
            .parse::<usize>()
            .expect("Could not parse ngram value!");

        let min_length = match matches.value_of("min_length") {
            Some(min) => min.parse::<usize>()
                .expect("Could not parse min_length value!"),
            None => 1
        };

        // TODO: turn this into a parameter
        let input = open_file("input.txt").expect("Could not read input file");
        File::create(Path::new("output.txt")).expect("Could not create new output file");
        Markov::new(ngram_size, min_length)
            .with_corpus(&input)
            .build()
            .generate_many_new(count, true);
        println!("Done.");
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let count: usize = matches.value_of("COUNT")
            .unwrap()
            .parse::<usize>()
            .expect("Could not parse <COUNT> value!");

        match open_file("output.txt") {
            Ok(output) => {
                let mut rng = thread_rng();
                if output.is_empty() {
                    println!("output.txt is empty! You need to run the\"generate\" subcommand first.");
                    return
                }

                for _ in 0..count {
                    let choice = output.choose(&mut rng).expect("Could not read from output vector!");
                    println!("{}", choice);
                }
            },

            Err(_) => {
                println!("output.txt does not exist! You need to run the\"generate\" subcommand first.");
                return;
            }
        }
    }
}

pub struct Markov<'a> {
    // size of n-grams
    pub ngram_size: usize,
    pub minimum_length: usize,
    pub map: HashMap<String, Vec<Option<char>>>,
    corpus: Option<&'a Vec<String>>,
}

impl<'a> Markov<'a> {
    pub fn new(ngram_size: usize, minimum_length: usize) -> Self {
        Markov {
            ngram_size,
            minimum_length,
            map: HashMap::new(),
            corpus: None,
        }
    }

    pub fn generate_many_new(&self, num_results: usize, write: bool) -> Vec<String> {

        let mut results = Vec::new();
        println!("Generating...");
        for _ in 0..num_results {
            let new_name = self.generate();
            results.push(new_name);
        }

        println!("Removing duplicates...");
        results.sort();
        results.dedup();

        while results.len() < num_results {
            results.append(&mut self.generate_many_new( num_results - results.len(), false));
            results.sort();
            results.dedup();
        }

        if write {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("output.txt")
                .expect("Could not open output file to write!");
            println!("Writing to file...");
            for result in &results {
                writeln!(file, "{}", &result).expect("Could not write to file!");
            }
        }

        results
    }

    pub fn generate(&self) -> String {


        let mut rng = thread_rng();

        let mut key = " ".to_string();
        loop {
            match key
                .chars()
                .next()
                .expect("Could not get beginning char!")
                .is_alphanumeric()
            {
                // retry if starts with any of these
                 false => key = self.map
                    .keys()
                    .choose(&mut rng)
                    .expect("Could not select key!")
                    .clone(),

                true => break
            }
        }

        if key == " ".to_string() {
            panic!("Input must have at least 1 line starting with a letter or number");
        }

        let mut result = key.clone();

        loop {
            let value = *self.map[&key]
                .choose(&mut rng)
                .expect("Could not select value!");

            match value {
                Some(c) => {
                    result = format!("{}{}", result, &c);
                    key = self.next_key(&key, c.clone());
                }
                None => break
            }
        };

        let first_char = result
            .chars()
            .next()
            .expect("Could not get beginning char!");
        if first_char == ' ' 
            || first_char == '.'
            || first_char == '-'
            || result.len() < self.minimum_length {
                result.clear();
                result = self.generate()
        }


        let mut chars: Vec<char> = result.chars().collect();
        for i in 0..chars.len() {
            if i > 0 {
                if chars[i - 1] == ' ' {
                    chars[i] = chars[i].to_ascii_uppercase();
                }
            }
        }

        let beginning = chars[0]
            .to_ascii_uppercase();

        let end: String = chars
            .split_off(1)
            .into_iter()
            .collect();

        let result = format!("{}{}", beginning, end);

        result.trim().to_string()
    }

    fn next_key(&self, key: &str, value: char) -> String {
        let mut last = key.to_string();
        last.remove(0);
        last.push(value);
        last
    }

    pub fn with_corpus(mut self, corpus: &'a Vec<String>) -> Self {
        self.add_corpus(corpus);
        self
    }

    pub fn add_corpus(&mut self, corpus: &'a Vec<String>) {
        self.corpus = Some(corpus)
    }

    pub fn build(mut self) -> Self {
        let corpus = self.corpus.expect("No corpus added!");
        for string in corpus.iter() {

            let chars: Vec<char> = string.chars().collect();
            for i in 0..=chars.len() - self.ngram_size {
                let mut key = String::new();

                for j in 0..self.ngram_size {
                    key.push(chars[i + j]);
                }

                if i == chars.len() - self.ngram_size {
                    self.insert(key, None);
                    continue;
                }

                let value = chars[i + self.ngram_size];
                self.insert(key,Some(value));
            }
        }

        self
    }

    fn insert(&mut self, key: String, value: Option<char>) {
        if self.map.contains_key(&key) {
            self.map.get_mut(&key)
                .expect(&format!("Could not get key: {}", &key))
                .push(value);
        }
        else {
            self.map.insert(key, vec!(value));
        }
    }
}

fn open_file(pathname: &str) -> Result<Vec<String>, io::Error> {
    use io::BufReader;

    let input_file = File::open(pathname)?;
    let reader = BufReader::new(input_file);
    let output: Result<Vec<_>, _> = reader
        .lines()
        .collect();
    output
}
