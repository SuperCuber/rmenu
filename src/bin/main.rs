extern crate rmenu;

fn main() {
    let ans = rmenu::iter(identity);
    println!("Final: {:?}", ans);
}

fn identity(text: &str) -> String {
    println!("Processing: {}", text);
    String::from(text)
}

