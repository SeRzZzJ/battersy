use battersy::run;

use std::process;

fn main() {
    run().unwrap_or_else(|err| {
        println!("The Error: {err}");
        process::exit(1);
    });
}
