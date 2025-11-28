use std::{error::Error,};

pub fn run_bubble_sort(args: Vec<String>) -> Result<(), Box<dyn Error>> {

    let mut custom_input = false;
    let mut default_input = false;
    let mut show_steps = false;
    
    if args.len() < 1 {
        return Err("Invalid number of arguments".into());
    }

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-d" => {
                default_input = true;
                custom_input = false;
            },
            "-i" => {
                custom_input = true;
                default_input = false;
                if index + 1 < args.len() && args[index + 1].starts_with('[') && args[index + 1].ends_with(']'){
                    index += 1;
                }
                else {
                     return Err("Missing the custom input argument '[...]'".into());
                }
            },
            "-s" => {
                show_steps = true;
            }
            _ => {
                return Err("Argument not valid type 'dsa' for help".into());
            }
        }
        index += 1;
    }

    if default_input {

    }

    else if custom_input {

    }

    println!("Running bubble sort");
    Ok(())
}
