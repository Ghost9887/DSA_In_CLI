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

        let mut default_arr = vec![30, 21, 6, 78, 92, 14, 50, 33, 68];
        let mut moved = true;
        let mut step = 1;
        while moved {
            moved = false;
            for i in 0..default_arr.len() - 1{

                if i + 1 > default_arr.len() - 1 {
                    break;
                }
                
                let a = default_arr[i];
                let b = default_arr[i + 1];

                if a > b {
                    default_arr[i] = b;
                    default_arr[i + 1] = a;
                    moved = true;
                }            
            }
            if show_steps {
                println!("{}:", step);
                print!("");
                println!("{:?}\n", default_arr);
                //clear screen
                //print!("\x1B[2J\x1B[1;1H");
                step += 1;
            }
        }
        println!("Result: {:?}", default_arr);
        return Ok(());
    }

    else if custom_input {

    }
    Ok(())
}
