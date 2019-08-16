use std::env::*;
use std::path::PathBuf;
use std::path::Path;

use std::io::prelude::*;
use std::io::BufReader;
//use std::fs;
use std::io;
use std::fs::{self, DirEntry};


fn main()
{
    println!("Hello, world!");

    let args : Vec<String> = args().collect();
    if args.len() != 3
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

    println!("in {}", in_dir);

    let _fmt = &param1[idx0+2..];
    let _out_dir = &args[2];
    
    println!("out {}", _out_dir);

    let outDir : &Path = Path::new(_out_dir);
    let inDir : &Path = Path::new(in_dir);
    visit_dirs(inDir, _fmt, outDir, &cp00);
}

fn cp00(dir : &DirEntry, sufifx: &str, dest_dir: &Path) -> () {
    //println!("{}", sufifx);

    let path : PathBuf = dir.path();
    let spath = path.into_os_string();    
    let string = spath.to_str();
    match string {    
        Some(string) => {
            if string.ends_with(sufifx) {
                println!("{:?} --> {} ", string, dest_dir.display());
                fs::copy(string, dest_dir);                    
            }
        },
        _=> {}
    }
}

fn visit_dirs(dir: &Path, suffix: &str, dest_dir : &Path, cb: &dyn Fn(&DirEntry, &str, &Path)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, suffix, dest_dir, cb )?;
            } else {
                cb(&entry, &suffix, dest_dir);
            }
        }
    }
    Ok(())
}