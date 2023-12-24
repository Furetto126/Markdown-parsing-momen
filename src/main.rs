mod scanner;
mod token;

#[cfg(test)]
mod tests {
    pub mod scanner_test;
}

use scanner::Scanner;

fn main() {
    let input = "**emojiii** ciao # h \n".to_string();

    for t in Scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
