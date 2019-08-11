use std::env::*;
use std::path::PathBuf;
use std::path::Path;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs;

fn main()
{
    println!("Hello, world!");

    let args : Vec<String> = args().collect();
    if args.len() != 4 
    {
        println!("para error");
        //return;
    }

    
    let path = current_dir().expect("cur dir is illegal, curiouslly!");
    println!( "{:?} ", path);

    let param1 = &args[1];
    println!("{}", param1);

    let mut idx = param1.rfind('\\');
    if idx == None
    {
        idx = param1.rfind('/');
    }    
    let idx0 = idx.expect("need a path");
    let in_dir = param1.split_at(idx0).0;

    println!("{}", in_dir);

    let fmt = &param1[idx0..];
    let _out_dir = &args[2];

    if let entry = fs::read_dir(in_dir) {        
        for _i in entry
        {
            let dir = entry;        
            println!("{:?}", Some(dir));
            //fs::copy(dir, _out_dir);
        }    
    }
}