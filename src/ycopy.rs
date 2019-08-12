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
    if args.len() != 3
    {
        println!("para error");
        //return;
    }

    //Result
    

    
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
    
    println!("{}", _out_dir);

    if let entry = fs::read_dir(in_dir).expect("read_dir error") {        
        let dir = entry;        
        println!("{:?}", Some(dir));

      /*  if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                ;//visit_dirs(&path, cb)?;
            } else {
                //cb(&entry);
                 println!("{:?}", Some(dir));
            }
        }
        */
        //fs::copy(dir, _out_dir);
    }
}