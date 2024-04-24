mod types;
mod program;
mod input;

use program::Program;
use types::Sym;

use std::{env, fs::File, io::{self, Read}};

fn read_code(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_program(file: String) -> Result<Vec<Sym>, String> {
    let mut program = Vec::new();

    let mut stack: Vec<u64> = Vec::new();

    let mut comment: bool = false;
    let mut ip: u64 = 0;
    for c in file.chars() {
        // Parse comment
        if comment {
            if c == '\n' {
                comment = false;
            }
            continue;
        } else if c == ';' {
            comment = true;
            continue;
        }

        // Parse symbols
        let sym = match c {
            '>' => Sym::IncPtr,
            '<' => Sym::DecPtr,
            '+' => Sym::Inc,
            '-' => Sym::Dec,
            '.' => Sym::WriteOut,
            ',' => Sym::ReadIn,
            '[' => {
                stack.push(ip);
                Sym::Begin(0)
            },
            ']' => {
                if let Some(begin_ip) = stack.pop() {
                    program[begin_ip as usize] = Sym::Begin(ip - 1);
                    Sym::End(begin_ip)
                } else {
                    return Err(format!("Failed to parse program, char '{}' at IP {} unexpected", c, ip));
                }
            },
            // Invalid symbols are basically just whitespace in regards to the spec
            _ => continue,
        };

        program.push(sym);
        ip += 1;
    }

    Ok(program)
}

fn get_filename() -> (String, Option<String>) {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        Some(args[1].to_owned())
    } else {
        None
    };

    (args[0].to_owned(), filename)
}

fn main() {
    let (exe_name, filename) = get_filename();
    if filename.is_none() {
        println!("Usage: {} <filepath>", exe_name);
        return;
    }

    let filename = filename.unwrap();
    let code = read_code(filename.as_str());
    if let Err(e) = code {
        eprintln!("Failed to read code file: {}", e);
        return;
    }

    let code = code.unwrap();
    let program = parse_program(code);
    if let Err(e) = program {
        eprintln!("Failed to parse program: {}", e);
        return;
    }

    let mut program = Program::new(program.unwrap());
    if let Err(e) = program.run() {
        eprintln!("Failed executing program: {}", e);
    }
}
