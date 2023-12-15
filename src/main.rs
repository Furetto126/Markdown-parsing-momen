mod scanner;
mod token;

fn main() {
    let input = r#"# t"#.to_string();

    for t in scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
