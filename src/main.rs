fn main() {
    println!("Running Collatz conjecture...");
    let args: Vec<String> = std::env::args().collect();
    let mut first_arg: u64 = args[1].parse().unwrap();
    let mut to_check: u64 = 1;
    if args.len() > 2 {
        println!("`{}`", args[2]);
        if args[2] == "--prove" {
            while first_arg >= to_check {
                let mut checking: u64 = to_check;
                while checking != 1 {
                    if checking % 2 != 0 {
                        checking = checking * 3 + 1
                    } else {
                        checking = checking / 2
                    }
                }
                println!("{}", checking);
                to_check += 1;
            }
        }
    }
    while first_arg != 1 {
        if first_arg % 2 != 0 {
            first_arg = first_arg * 3 + 1
        } else {
            first_arg = first_arg / 2
        }
        println!("{0}", first_arg)
    }
}
