mod scanner;
mod token;

fn main() {
    let input = "**emojiii** ciao # h \n".to_string();

    for t in scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
