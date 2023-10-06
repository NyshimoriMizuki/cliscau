use std::io::Write;

mod core;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments.\nType -h or --help for more information of usege");
    } else if args.get(1).unwrap() == "--loop" {
        let mut interpreter = core::MathIntepreter::new();
        loop {
            let mut cmd = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut cmd).unwrap();

            if is_in(cmd.trim(), vec!["exit", "quit", "e"]) {
                break;
            } else if is_in(&cmd.trim(), vec!["show-vars", "sv"]) {
                for var in interpreter.get_vars() {
                    println!("{} = {};", var.0, var.1);
                }
            } else {
                interpreter.read(&cmd.trim());
            }
        }
    } else if args.get(1).unwrap() == "-h" || args.get(1).unwrap() == "--help" {
        println!("test");
    } else {
        let mut interpreter = core::MathIntepreter::new();
        interpreter.read(args.get(1).unwrap())
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
