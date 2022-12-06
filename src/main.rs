use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1..] {
        [] =>
            repl(),
        [path] =>
            interpret(path),
        _ =>
            panic!("unknown args: {:?}", args),
    }
}

fn repl() {
    let mut buf = String::new();
    loop {
        buf.clear();
        print!("> ");
        io::stdout().flush().expect("flush stdout");
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                buf.pop(); // pop newline
                if buf == ":q" {
                    break;
                }
                // TODO
                println!("eval input: {}", buf)
            },
            Err(e) =>
                panic!("Fail to read from stdin: {}", e),
        }
    }
}

fn interpret(path: &str) {
    match fs::read_to_string(&path) {
        Ok(source) => {
            todo!("interpret file {}, file size = {}", path, source.len());
        },
        Err(e) =>
            panic!("Fail to read file: {}, error: {}", path, e),
    }
}
