mod scanner;
mod token;

fn main() {
    let input = "# ciao".to_string();

    for t in scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
