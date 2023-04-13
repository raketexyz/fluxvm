use std::env;
use std::fs::File;
use std::io::Read;

use fluxvm::vm::VM;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} input_file", args[0]));
    }

    let input_file = &args[1];

    let mut binary = Vec::new();
    let mut file = File::open(input_file).map_err(|e| e.to_string())?;
    file.read_to_end(&mut binary).map_err(|e| e.to_string())?;

    let mut vm = VM::new(binary);
    if let Err(s) = vm.execute() {
        println!("Error: {}", s);
        Err(s)?;
    }

    Ok(())
}
