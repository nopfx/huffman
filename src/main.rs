mod args;
mod decode;
mod encode;
mod htree;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = args::Param::new(args);

    let action = params.action();
    let file_in = params.flag::<String>("in").unwrap_or("".into());
    let file_to = params
        .flag::<String>("out")
        .unwrap_or(format!("{}_compressed", file_in));

    if !Path::new(&file_in).exists() || action == args::Action::Help {
        args::help();
        std::process::exit(1)
    }

    let input = fs::read(file_in).expect("Unable to read a file");

    let data = match action {
        args::Action::Encode => encode::data(&input),
        args::Action::Decode => decode::data(&input),
        _ => vec![],
    };

    let mut file = File::create(file_to).expect("Cannot create out file");
    file.write_all(&data).expect("Cannot to write to out file");
}
