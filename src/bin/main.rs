extern crate rmenu;

fn main() {
    let mut options = Vec::new();

    options.push(("1", "Test"));
    options.push(("2", "Test"));
    options.push(("3", "Test"));
    options.push(("4", "Test"));

    let mut converted_options = Vec::new();
    for entry in options {
        converted_options.push((entry.0.into(), entry.1.into()));
    }

    let ans = rmenu::run(|s| process(s, &converted_options));
    println!("Final: {:?}", ans);
}

fn filter(text: &str, options: &[(String, String)]) -> Vec<(String, String)> {
    let mut answer = Vec::new();
    for option in options {
        if option.0.starts_with(text) {
            answer.push(option.clone());
        }
    }
    answer
}

fn process(text: &str, options: &[(String, String)]) -> Vec<String> {
    let tuple_answer = filter(text, options);
    let mut text_answer = Vec::new();
    for (key, value) in tuple_answer {
        let mut text = String::new();
        text.push_str(&key);
        text.push_str(" - ");
        text.push_str(&value);
        text_answer.push(text);
    }
    text_answer
}
