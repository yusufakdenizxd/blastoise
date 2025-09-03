use crate::{env::Env, parser::Parser};
use std::io::Write;

pub fn run() {
    let home = std::env::home_dir().expect("Home is missing");
    let mut env = Env::new(home);
    loop {
        let mut input = String::new();

        print!("{} > ", env.dirpath.display());
        std::io::stdout().flush().expect("An error Occured");
        std::io::stdin()
            .read_line(&mut input)
            .expect("An Error Occured");
        let parser = Parser::new(input);
        let command = parser.parse();
        match command {
            Ok(a) => {
                let result = a.execute(&mut env);
                if let Err(e) = result {
                    println!("{:?}", e);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
