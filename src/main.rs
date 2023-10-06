use std::io::Write;

mod core;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments.\nType -h or --help for more information of usege");
    } else if args.get(1).unwrap() == "--loop" {
        print_opening_message();

        let mut interpreter = core::MathIntepreter::new();
        let mut counter = 0;

        loop {
            let mut cmd = String::new();
            print!("[{}]> ", counter);
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut cmd).unwrap();

            if is_in(cmd.trim(), vec!["!exit", "!quit", "!e", "!q"]) {
                break;
            } else if is_in(&cmd.trim(), vec!["!show-vars", "!sv"]) {
                let vars = interpreter.get_vars();
                if vars.is_empty() {
                    println!("No vars found.\n");
                } else {
                    for var in vars {
                        println!("{} = {};", var.0, var.1);
                    }
                }
            } else if is_in(&cmd.trim(), vec!["!help", "!h"]) {
                print_loop_help_message();
            } else {
                interpreter.read(&cmd.trim());
            }

            counter += 1;
        }
    } else if is_in(args.get(1).unwrap(), vec!["-h", "--help"]) {
        print_cli_help_message();
    } else {
        let mut interpreter = core::MathIntepreter::new();
        interpreter.read(args.get(1).unwrap().trim())
    }
}

fn is_in(value: &str, targets: Vec<&str>) -> bool {
    for i in targets.iter() {
        if value == *i {
            return true;
        }
    }
    false
}

fn print_opening_message() {
    println!("Cliscau - CLI Simple Cauculator (v0.1.0 - 10/23)");
    println!("Type \"!h\" for more information.\n");
}

fn print_loop_help_message() {
    println!("Type some math expressions, like \"(1+2)*3\".\n");
    println!("Commands:");
    println!("\t- quit the program: !quit !exit !q !e");
    println!("\t- show all defined variables: !show-vars !sv");
    println!("");
}

fn print_cli_help_message() {
    println!("Cliscau - CLI Simple Cauculator\n");
    println!("Usage: cliscau [COMMAND]\n");
    println!("Commands:");
    println!("\t\"1+2\" - you can type a simple math expression inside the quotes.");
    println!("\t--loop - start the Cliscau as a Repl.");
    println!("");
}
