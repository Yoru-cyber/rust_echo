use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut no_newline: bool = false;
    if args.len() <= 1 {
        return;
    }
    for arg in args {
        match arg.as_str() {
            "-n" => no_newline = true,
            _ => print!("{} ", arg),
        }
    }
    if no_newline == true {
        print!("");
        return;
    }
    print!("\n");
    return;
}
