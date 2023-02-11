use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        print!("> ");
        _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        //let command = parts.next().unwrap();
        if let Some(command) = parts.next() {
            let args = parts;
            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "exit" => return,
                command => {
                    let child = Command::new(command).args(args).spawn();
                    match child {
                        Ok(mut child) => {
                            _ = child.wait();
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
    }
}
