mod scanner;
mod token;

#[cfg(test)]
mod tests {
    pub mod scanner_test;
}

fn main() {
    let input = "**emojiii** ciao # h \n".to_string();

    for t in scanner::parse(input).tokens {
        println!("{:?}", t);
    }
}
