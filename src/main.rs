use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use serde_json::json;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = if let Some(p) = args.get(1) {
        Path::new(p)
    } else {
        eprintln!("please provide a file path!");
        return
    };

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(why) => {
            eprintln!("couldn't open {}: {}", path.display(), why);
            return
        },
    };

    let mut content = String::new();
    if let Err(why) = file.read_to_string(&mut content) {
        eprintln!("couldn't read {}: {}", path.display(), why);
        return
    }

    let data: Value = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(error) => {
            eprintln!("serde_json parse error: {}", error);
            return
        }
    };

    println!("{}", data);
}
