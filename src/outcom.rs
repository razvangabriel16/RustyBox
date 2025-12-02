use std::{fs::File, io::{self, BufRead, BufReader}};

const MAX_LINES_THRESHOLD: usize = 1000;

fn count_lines(path: &str) -> io::Result<usize> {
    let mut count = 0;
    for line in BufReader::new(File::open(&path)?).lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

pub(crate) fn help(arguments: &[String]){
    println!("Help function called with arguments: {:?}", arguments);
}

pub(crate) fn echo(arguments: &[String]) {
    let mut printBin = false;
    let mut printHex = false;
    let mut no_newline = false;
    let mut start_index = 0;
    while let Some(arg) = arguments.get(start_index) {
        match arg.as_str() {
            "-n" | "--no-newline" => { no_newline = true; start_index += 1; }
            "-hex" | "--print-hex" => { printHex = true; start_index += 1; }
            "-bin" | "--print-bin" => { printBin = true; start_index += 1; }
            _ => break,
        }
    }

    let mut first = true;

    for arg in &arguments[start_index..] {
        if !first {
            print!(" ");
        }
        first = false;
        print!("{}", arg);
        if let Ok(num) = arg.parse::<i32>() {
            if printHex {
                print!(" {:x}", num);
            }
            if printBin {
                print!(" {:b}", num);
            }
        }
    }
    if !no_newline {
        println!();
    } else {
        use std::io::{self, Write};
        io::stdout().flush().unwrap(); //flush() equivalent from C
    }
}

pub(crate) fn cat(arguments: &[String]) {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    println!("Files to cat: {:?}", arguments);
    if arguments.is_empty() {
        panic!("No file provided for cat command");
    }
    for path in arguments {
        match count_lines(path) {
            Ok(n) if n > MAX_LINES_THRESHOLD => {
                eprintln!(
                    "Warning: File '{}' has {} lines, which exceeds the threshold of {} lines.",
                    path, n, MAX_LINES_THRESHOLD
                );
            },
            Err(_) => {
                eprintln!("Failed to count lines for file '{}'", path);
                continue;
            },
            _ => {}
        }
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file '{}': {}", path, e);
                continue;
            }
        };
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading line in '{}': {}", path, e),
            }
        }
    }
}