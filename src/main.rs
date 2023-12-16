mod scanner;
mod token;

fn main() {
    //                               da problemi quando il previous token è un closer  
    let input = "**emojiii** aaaa \n# eee".to_string();

    for t in scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
