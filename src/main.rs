use text_io::read;

fn main() {
    println!("Enter number to run collatz conjecture on: ");
    let mut i: i64 = read!();
    while i != 1 {
        if i % 2 == 1 {
            i = i * 3 + 1
        } else {
            i = i / 2
        }
        println!("{0}", i)
    }
}
