use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use serde_json::json;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(path_str) = args.get(1) else {
        eprintln!("please provide a file path");
        return
    };
    let mut tmp_path_str = path_str.clone();
    tmp_path_str.push_str(".tmp");

    let path = Path::new(path_str);
    let tmp_path = Path::new(&tmp_path_str);

    let mut data: Value = if path.exists() {
        let mut file = match File::options().read(true).open(path) {
            Ok(f) => f,
            Err(error) => {
                eprintln!("couldn't open {}: {}", path.display(), error);
                return
            },
        };
    
        let mut content = String::new();
        if let Err(error) = file.read_to_string(&mut content) {
            eprintln!("couldn't read {}: {}", path.display(), error);
            return
        }
    
        match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(error) => {
                eprintln!("serde_json parse error: {}", error);
                return
            }
        }
    } else {
        json!({
            "n": 0,
            "name": "Violet"
        })
    };

    println!("{}", data["n"]);

    data["n"] = json!(data["n"].as_u64().expect("data.n should be int") + 1);

    let mut tmp_file = match File::options().write(true).create_new(true).open(tmp_path) {
        Ok(f) => f,
        Err(error) => {
            eprintln!("couldn't open {}: {}", path.display(), error);
            return
        },
    };

    if let Err(error) = tmp_file.write_all(serde_json::to_string(&data).unwrap().as_bytes()) {
        eprintln!("{}", error);
        drop(tmp_file);
        if let Err(error) = fs::remove_file(tmp_path_str) {
            eprintln!("failure to cleanup: {}", error)
        };
        return
    }

    if let Err(error) = std::fs::rename(&tmp_path, &path) {
        eprintln!("{}", error);
        drop(tmp_file);
        if let Err(error) = fs::remove_file(tmp_path_str) {
            eprintln!("failure to cleanup: {}", error)
        };
        return
    };
}
