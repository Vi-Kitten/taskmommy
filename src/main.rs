use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use tasks::*;

pub mod tasks;

struct ProcessMemory {
    selected_task_id: Option<TaskID>
}

impl ProcessMemory {
    fn new() -> ProcessMemory {
        ProcessMemory {
            selected_task_id: None
        }
    }

    fn try_parse_arg(&mut self, data: &mut Data, arg: &String) -> Result<(), String> {
        Ok(false)
        .and_then(|done| self.try_parse_new_task(data, arg, done))
        .and_then(|done| {
            if done {
                Ok(())
            } else {
                Err(format!("flag ({}) not recognised", arg))
            }
        })
    }

    fn try_parse_new_task(&mut self, data: &mut Data, arg: &String, done: bool) -> Result<bool, String> {
        if done { return Ok(done) }
        if let Some(rest) = arg.strip_prefix("-n") {
            let new_task = if rest == "" {
                tasks::Task::new()
            } else {
                let Some(base) = data.tasks.get(&rest.to_lowercase()) else {
                    return Err(format!("invalid id ({}) for base task", rest))
                };
                tasks::Task {
                    base: Some(format!("{}", rest)),
                    tags: base.tags.clone(),
                    name: None,
                    logs: vec![]
                }
            };

            data.tasks_created += 1;
            /*
                the key is generated by multiplying the counter value
                by the fractional component of the golden ratio scaled by max u32
                that is rounded to the nearest odd number to ensure it is coprime with max u32.
                this makes the key feel random
            */ 
            let id = format!("{:08x}", data.tasks_created.wrapping_mul(2654435769));
            data.tasks.insert(id.clone(), new_task);
            self.selected_task_id = Some(id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut iter_args = args.iter();

    let _app_location = iter_args.next().unwrap();

    let Some(path_str) = iter_args.next() else {
        eprintln!("please provide a file path");
        return
    };
    let mut tmp_path_str = path_str.clone();
    tmp_path_str.push_str(".tmp");

    let path = Path::new(path_str);
    let tmp_path = Path::new(&tmp_path_str);

    let mut data: Data = if path.exists() {
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
        tasks::Data::new()
    };

    println!("{}", data.tasks_created);

    let mut process_memory = ProcessMemory::new();

    if let Err(error) = iter_args.try_for_each(|arg| process_memory.try_parse_arg(&mut data, arg)) {
        print!("{}", error)
    }

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
