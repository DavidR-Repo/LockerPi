use std::env;

mod command_line; // bring command_line.rs into main.rs scope
mod debugging;

use command_line as cmdln;
use cmdln::{Cmd, /* SubCmdList, SubCmdAdd, SubCmdModify */};
use debugging as dbg;



fn main() {

    println!("Beginning program...\n");
    
    let use_fake_input = false;
    let debug = dbg::help();
    let mut cmd: Vec<String> = env::args().collect();
    
    for i in &cmd {
        print!("{} ", i);
    }
    println!();

    if cmd.len() <= 1 && use_fake_input {
        println!("--- using debug commands ---");

        cmd.clear();
        for i in debug{ 
            cmd.push(i.clone());
        }
        for i in &cmd {
            print!("{} ", i);
        }
    }
    println!();

    if cmd.len() > 1 {
        let c: Cmd = cmdln::parse_cmd(&cmd[1]);
        match c {
            Cmd::None => {
                println!("Error, invalid command: {}", &cmd[1]);
                println!("For available commands, use:  locker_pi.exe -help");
            },
            Cmd::Help => {
                println!("Command: {}", &cmd[1]);
                cmdln::handle_usage()
            },
            Cmd::List => {
                println!("Command: {}", &cmd[1]);
                cmdln::handle_list(cmd)
            },
            Cmd::Add => {
                println!("Command: {}", &cmd[1]);
                cmdln::handle_add(cmd)
            },
            Cmd::Remove => {
                println!("Command: {}", &cmd[1]);
                cmdln::handle_remove(cmd)
            },
            Cmd::Modify => {
                println!("Command: {}", &cmd[1]);
                cmdln::handle_modify(cmd)
            },
        }
    }else{
        println!("No arguments found");
        println!("For available commands, use:  locker_pi.exe -help");
    }
    

    println!("\nEnding program...\n");
}


