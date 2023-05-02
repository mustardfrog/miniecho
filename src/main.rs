use std::env;

fn main() {
    echo();
}

fn echo() {
    let args: Vec<_> = env::args().collect();
    let mut result = String::new();

    for i in args.iter().skip(1) {
        result.push_str(i);
        result.push_str(" ");
    }
    println!("{result}");
}
