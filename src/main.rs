use std::io::{ stdout, stdin, Write };
use std::process::{ Command, Stdio };
use std::env;
use std::path::Path;

fn main() {
    loop {
        let mut input = String::new();

        let output = Command::new("whoami").stdout(Stdio::piped()).output().unwrap();
        let username = String::from_utf8(output.stdout).unwrap();

        print!("{}> ", username.trim());
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                                .args(args)
                                .spawn();
                match child {
                    Ok(mut child) => { let _ = child.wait(); },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
