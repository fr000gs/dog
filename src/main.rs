use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::metadata;

/*
fn get_caret(mut string: String) -> String {
    let strup = vec![];
    for byte in string.as_bytes() {
        let aa = ascii::caret_encode(*byte);
        if aa.is_some() {
            strup.push(aa.unwrap().as_byte());
        }
        else {
            strup.push(byte.clone());
        };
    }
    std::str::from_utf8(strup).unwrap().to_string()
}
*/

fn options(mut file: String, args_1: Vec<String>) -> String {
    // TODO: Make a macro
    /*
    macro_rules! contains {
        ($args_1:expr, $content:expr) => {
            if $args_1.contains($content) {true}
            else {false}
        };
        ($args_1: expr, $content:expr, $($contents: expr),+) => {
            contains! { $args_1, $content }
            contains! { $args_1, $($contents),+ }
        };
    }
    */
    // TODO: Use macros
    if args_1.contains(&String::from("-E")) || args_1.contains(&String::from("--show-ends")) ||
        args_1.contains(&String::from("-A")) || args_1.contains(&String::from("--show-all")) ||
            args_1.contains(&String::from("-e")) {
                // Show line-ends
                file = file.replace('\n', "$\n");
            }
    if args_1.contains(&String::from("-s")) || args_1.contains(&String::from("--squeeze-blank")) {
        // Squeeze blank lines
        while file.contains(&String::from("\n\n\n")) {
            file = file.replace("\n\n\n", "\n\n");
        }
    }
    if args_1.contains(&String::from("-T")) || args_1.contains(&String::from("--show-tabs")) ||
        args_1.contains(&String::from("-A")) || args_1.contains(&String::from("--show-all")) {
            // Show tabs
            file = file.replace('\t', "^I");
        }
    //dbg!(ascii::caret_encode(b'\n').unwrap());
    file
}

fn hyphen(args_1: Vec<String>) {
    loop {
        let mut line = String::new();
        let bytes: usize = std::io::stdin()
            .read_line(&mut line)
            .unwrap();
        if bytes == 0 {break;}
        print!("{}", options(line, args_1.clone()));
        //print!("{}", options(line, args_1));
        std::io::stdout().flush().expect("Pwned!");
    }
}

fn parse_args(whole_args: Vec<String>) -> [Vec<String>; 2] {
    let args_1: Vec<String>;
    let only_filename_args: Vec<String>;
    let mut pos_2hyphen: usize = 0;
    {
        let i = whole_args.iter().position(|item| item =="--");
        if let Some(i_value) = i {pos_2hyphen=i_value}
    }
    if pos_2hyphen != 0 {
        args_1 = whole_args[1..pos_2hyphen].to_vec();
        only_filename_args = whole_args[(pos_2hyphen+1)..].to_vec();
    }
    else {
        args_1 = whole_args[1..].to_vec();
        only_filename_args = vec![];
    }
    [args_1, only_filename_args]
}

fn cat(filename: String, args_1: Vec<String>) -> bool {
    // TODO: optimise this function
    let meda = metadata(&filename);
    if meda.is_err() {
        eprintln!("dog: {}: No such file or directory", &filename);
        return true;
    }
    let myfile = if meda.unwrap().is_file() {
        //File::open(&filename).expect(&format!("dog: {}: No such file or directory", &filename))
        File::open(&filename).unwrap()
    }
    else {
        eprintln!("dog: {}: Is a directory", &filename);
        return true
    };
    let mut buf_reader = BufReader::new(myfile);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap_or_else(|_| panic!("Failed to read file: {}", &filename));
    print!("{}", options(contents, args_1));
    std::io::stdout().flush().expect("Failed to flush pwned");
    false
}

fn help() {
    println!(
"Usage: dog [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

(Double EOF is required instead of single EOF when
last character is not LF)

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n // TODO (after -n)
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines // TODO
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB // TODO
      --help        display this help and exit
      --version     output version information and exit

Examples:
  dog f - g  Output f's contents, then standard input, then g's contents.
  dog        Copy standard input to standard output. // TODO"); 
    std::process::exit(0);
}

fn version() {
    println!("
             dog (fr000gs dogsutil) 0.1.0
Copyright (C) fr000gs
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by fr000gs (Fat Frog).");
    std::process::exit(0);
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let argus = parse_args(args.clone());
    let (args_1, only_filename_args) = (argus[0].clone(), argus[1].clone());

    if args_1.contains(&String::from("--help")) || args_1.contains(&String::from("-h")) {
        help();
    }

    if args_1.contains(&String::from("--version")) || args_1.contains(&String::from("-v")) {
        version();
    }

    dbg!(args.len() < 2_usize);
    for filename in &args_1 {
        if filename.as_bytes() == [b'-'] || args.len() < 2_usize {
            // FIXME
            hyphen(args_1.clone());
        }
        else if filename.as_bytes()[0]!=b'-' && cat(filename.to_string(), args_1.clone()) {
            continue;
        }
    }
    for filename in &only_filename_args {
        if cat(filename.to_string(), args_1.clone()) {
            continue;
        }
    }
}
