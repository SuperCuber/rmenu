extern crate rmenu;

use std::collections::HashMap;

fn main() {
    let mut options = HashMap::<&str, &str>::new();

    options.insert("Hi", "Bye");

    let mut converted_options = HashMap::new();
    for entry in options {
        converted_options.insert(entry.0.into(), entry.1.into());
    }

    let ans = rmenu::run(|s| process(s, &converted_options));
    println!("Final: {:?}", ans);
}

fn filter(text: &str, options: &HashMap<String, String>) -> HashMap<String, String> {
    let mut answer = options.clone();
    answer.retain(|k, _| k.starts_with(text));
    answer
}

fn process(text: &str, options: &HashMap<String, String>) -> String {
    let answer = filter(text, options);
    if answer.is_empty() {
        return String::new();
    }
    let mut text_answer = String::new();
    for (key, value) in answer {
        text_answer.push_str(&key);
        text_answer.push_str(" - ");
        text_answer.push_str(&value);
        text_answer.push('\n');
    }
    String::from(&text_answer[..text_answer.len() - 1])
}
