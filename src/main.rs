use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word_list_to_check_filename = &args[1];
    let mut no_prefix_list_output = format!("{}.no-prefix", &word_list_to_check_filename);
    if args.len() == 3 {
        no_prefix_list_output = args[2].to_string();
    }
    let prefix_words = split_and_search(
        make_vec(word_list_to_check_filename),
        word_list_to_check_filename,
    );
    // let words_to_remove = find_words_to_remove(single_bad_words, double_bad_words, must_remove_words);

    println!("Making no-prefix list");
    let clean_word_list = make_clean_list(prefix_words, make_vec(word_list_to_check_filename));

    let mut f = File::create(&no_prefix_list_output).expect("Unable to create file");
    for i in &clean_word_list {
        writeln!(f, "{}", i).expect("Unable to write data to file");
    }

    println!("");
    println!("------------------------");
    println!("");
    let original_list_length = make_vec(word_list_to_check_filename).len() as f64;
    let clean_list_length = clean_word_list.len() as f64;
    println!(
        "You're inputted word list had {} words ({} bits per word).",
        original_list_length,
        log_base(2, original_list_length)
    );
    println!("");
    if clean_list_length == original_list_length {
        println!("I didn't find any problematic words. Your inputted word list appears to be compound-safe as is!");
    } else {
        println!(
            "The compound-safe list I made has {} words ({} bits per word). It's located at '{}'",
            clean_list_length,
            log_base(2, clean_list_length),
            &no_prefix_list_output
        );
    }
}

fn make_vec(filename: &str) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    return word_list;
}

fn split_and_search(words: Vec<String>, word_list_to_check_filename: &str) -> (Vec<String>) {
    let mut prefix_words: Vec<String> = [].to_vec();
    for mut word in words {
        println!("Starting search of {}", word);
        let mut second_half = "".to_string();
        for _i in 0..word.len() {
            let length = &word.len();
            second_half = format!("{}{}", &word.split_off(length - 1), second_half);
            if search(&word, word_list_to_check_filename) {
                println!(
                    "From {}{}, I found {} as its own word, and am adding it to prefix_words list",
                    word, second_half, word
                );
                prefix_words.push(word.to_string());
            }
        }
    }
    prefix_words
}

fn search(target_word: &str, word_list_to_check_filename: &str) -> bool {
    let words = make_vec(&word_list_to_check_filename);
    for word in words {
        if target_word == word {
            return true;
        }
    }
    return false;
}

fn make_clean_list(words_to_remove: Vec<String>, original_list: Vec<String>) -> Vec<String> {
    let mut clean_words: Vec<String> = [].to_vec();
    for original_word in original_list {
        let mut bad_word = false;
        for word_to_remove in &words_to_remove {
            if word_to_remove == &original_word {
                bad_word = true;
            }
        }
        if bad_word == false {
            clean_words.push(original_word);
        }
    }
    clean_words.sort();
    clean_words
}

fn log_base(base: u64, n: f64) -> f64 {
    let base_as_float: f64 = base as f64;
    return (n.ln() / base_as_float.ln()) as f64;
}
