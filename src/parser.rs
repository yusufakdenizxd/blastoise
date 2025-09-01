use anyhow::{anyhow, Result};
use whoami;

use crate::env::Env;

#[derive(Clone, Debug)]
pub enum Command {
    Empty,
    Exit,
    Echo(String),
    Ls,
    Pwd,
    Cd(Option<String>),
    Touch(Vec<String>),
    Rm(Vec<String>),
    Cat(String),
    Whoami,
}

impl Command {
    pub fn execute(&self, env: &mut Env) -> Result<()> {
        match self {
            Command::Exit => {
                std::process::exit(0);
            }

            Command::Echo(value) => {
                println!("{}", value);
                Ok(())
            }

            Command::Whoami => {
                println!("{}", whoami::username());
                Ok(())
            }

            Command::Cat(file) => {
                let mut path = env.dirpath.clone();
                path.push(file);
                let result = std::fs::read_to_string(&path);
                if result.is_ok() {
                    println!("{}", result.unwrap());
                }
                Ok(())
            }

            Command::Pwd => {
                println!("{}", env.dirpath.display());
                Ok(())
            }

            Command::Ls => {
                let files = env.dirpath.read_dir();
                match files {
                    Ok(files) => {
                        for file in files {
                            let display_name = file.unwrap().file_name();
                            println!("{}", display_name.to_str().unwrap());
                        }
                        Ok(())
                    }
                    Err(e) => Err(anyhow!("An Error Occured {}", e)),
                }
            }

            Command::Rm(files) => {
                for file in files {
                    let mut path = env.dirpath.clone();
                    path.push(file);
                    if path.exists() {
                        if path.is_file() {
                            let _ = std::fs::remove_file(path);
                        } else if path.is_dir() {
                            return Err(anyhow!("{} is a Folder", file));
                        }
                    } else {
                        return Err(anyhow!("{} file not exist ", file));
                    }
                }

                Ok(())
            }

            Command::Touch(files) => {
                for file in files {
                    let mut path = env.dirpath.clone();
                    path.push(file);
                    if path.exists() {
                        if path.is_dir() {
                            return Err(anyhow!("There is a Folder Called {}", file));
                        }
                        return Err(anyhow!("There is a File Called {}", file));
                    }

                    let _ = std::fs::write(&path, "");
                }

                Ok(())
            }

            Command::Cd(new) => {
                if new.is_none() {
                    env.dirpath.pop();

                    return Ok(());
                }
                let new = new.clone().unwrap();
                let mut new_path = env.dirpath.clone();
                new_path.push(&new);
                let is_directory = new_path.is_dir();
                if !is_directory {
                    return Err(anyhow!("{} Is not a Directory", new));
                }
                env.dirpath.push(&new);

                Ok(())
            }

            _ => Ok(()),
        }
    }
}

pub struct Parser {
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            input: String::from(input.trim()),
        }
    }

    pub fn parse(&self) -> Result<Command> {
        let args: Vec<String> = self.input.split_whitespace().map(String::from).collect();
        if args.len() == 0 {
            return Ok(Command::Empty);
        }

        match args[0].as_str() {
            "exit" => Ok(Command::Exit),
            "ls" => Ok(Command::Ls),
            "whoami" => Ok(Command::Whoami),
            "pwd" => Ok(Command::Pwd),
            "cd" => {
                if args.len() == 1 {
                    return Ok(Command::Cd(None));
                } else if args.len() == 2 {
                    return Ok(Command::Cd(Some(args[1].clone())));
                }
                Err(anyhow!("cd command requires an argument"))
            }
            "echo" => {
                if args.len() > 1 {
                    Ok(Command::Echo(args[1..].join(" ")))
                } else {
                    Err(anyhow!("echo command requires an argument"))
                }
            }
            "rm" => {
                if args.len() > 1 {
                    Ok(Command::Rm(args.into_iter().skip(1).collect()))
                } else {
                    Err(anyhow!("rm command requires an argument"))
                }
            }
            "touch" => {
                if args.len() > 1 {
                    Ok(Command::Touch(args.into_iter().skip(1).collect()))
                } else {
                    Err(anyhow!("touch command requires an argument"))
                }
            }
            "cat" => {
                if args.len() == 2 {
                    Ok(Command::Cat(String::from(args[1].clone())))
                } else {
                    Err(anyhow!("cat command requires a file name"))
                }
            }
            _ => Err(anyhow!("Unknown Command")),
        }
    }
}
