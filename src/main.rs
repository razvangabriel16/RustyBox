mod outcom;
mod filesys;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let command = match arguments.get(1){
        Some(cmd) => cmd,
        None => {
            eprintln!("No command provided"); //std error output
            std::process::exit(1);
            //return;
            //or panic!(...smth);
        }
    };
    match command.as_str() {
        "echo" => {
            println!("<echo> command selected");
            outcom::echo(&arguments[2..]);
        },
        "cat" => {
            println!("<cat> command selected");
            outcom::cat(&arguments[2..]);
        },
        "help" => {
            println!("<help> command selected");
            outcom::help(&arguments[2..]);
        },
        "chmod" => {
            println!("<chmod command selected>");
            filesys::chmod(&arguments[2], &arguments[3]);
        },
        "cp" => {
            println!("<cp command selected>");
            filesys::cp(&arguments[2..]);
        }
        ,
        _ => {
            panic!("Unknown command");
        },
    }

}
