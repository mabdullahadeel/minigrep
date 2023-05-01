use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = run(config) {
            println!("Run failed with error: {err}");
            process::exit(1)
    } else {
        eprintln!("\n process ran successfully")
    }
}
