// SPDX-License-Identifier: MIT
// Copyright(c) 2023 Itsuro Oda
// https://opensource.org/license/mit/

#![allow(dead_code)]

use std::{fs, io, process};
use std::io::Write;
use clap::Parser;

mod bytecode;
mod exec;
mod inst;
mod module;

#[derive(Parser)]
struct Args {
    /// path of wasm binary module
    path: String,

    /// show section detail
    #[arg(short)]
    sec: bool,

    /// show function code disassemble
    #[arg(short)]
    dis: bool,

    /// interactive (typically to use to exec functions)
    #[arg(short)]
    intr: bool,
}

fn main() {
    let args = Args::parse();

    let opts = args.sec as i32 + args.dis as i32 + args.intr as i32;
    if opts > 1 {
        eprintln!("-s, -d, -i cannot be specified at the same time.");
        process::exit(1);
    }

    let buf = fs::read(&args.path).unwrap_or_else(|err| {
        eprintln!("Read from '{}' failed: {}", &args.path, err);
        process::exit(1);
    });

    let module = module::init_module(buf).unwrap_or_else(|err| {
        eprintln!("init module failed: {}", err);
        process::exit(1);
    });

    if opts == 0 || args.intr {
        module.show_summary();
    } else if args.sec {
        module.show_section();
    } else {
        module.show_code();
    }

    if !args.intr {
        process::exit(0);
    }

    let mut store = exec::make_store(&module);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); 

        let cmds: Vec<&str> = line.split_whitespace().collect();
        if cmds.len() > 0 {
            match cmds[0] {
                "exec" => {
                    match exec::exec_func(&cmds[1..], &module, &mut store) {
                        Err(e) => eprintln!("error: {}", e),
                        Ok(_) => (),
                    }
                },
                "help" => print_help(),
                "exit" => break,
                _ => {
                    eprintln!("unknown command '{}'", cmds[0]);
                    print_help();
                },
            }
        }
    }

    process::exit(0);
}

fn print_help() {
    println!("exec funcidx [args..]");
    println!("help");
    println!("exit");
}
