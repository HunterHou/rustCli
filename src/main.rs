
use std::process::Command;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomErr(String);

fn main() -> Result<(), CustomErr> {
    println!("Hello, world!");
    let pattern = std::env::args().nth(1).expect("no word given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("args.patten:{}", args.pattern);
    println!("args.path:{:?}", args.path);
    let mut temp_dir = "";
    match args.path.to_str() {
        Some(st) => {
            println!("temp_dir:{:?}", st);
            temp_dir = st
        }
        _ => println!("dir err"),
    }
    // let result = std::fs::read_to_string(temp_dir);
    // let context = match result {
    //     Ok(context) => context,
    //     Err(error) => {
    //         return Err(error.into());
    //     }
    // };
    let context = std::fs::read_to_string(temp_dir)
        .map_err(|err| CustomErr(format!("read file {} file err:{}", temp_dir, err)))?;
    println!("context:{:?}", context);
    println!("Game Over!");

    let status=Command::new("cmd").current_dir("D:\\").arg("start ").arg(temp_dir).status();
    println!("excute result:{:?}",status);
    return Ok(());
}
