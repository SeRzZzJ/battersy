use battersy::run;

use std::{env, process};

#[cfg(target_os = "linux")]
fn main() {
    let mut args = env::args().skip(1);

    run(&mut args).unwrap_or_else(|err| {
        println!("The Error: {err}");
        process::exit(1);
    });
}
