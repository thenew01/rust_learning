use std::env::*;
use std::path::PathBuf;
use std::path::Path;

//use std::io::prelude::*;
//use std::io::BufReader;
//use std::fs;
use std::io;
use std::fs::{self, DirEntry};


fn main()
{
    let args : Vec<String> = args().collect();
    if args.len() != 3 && args.len() != 4 {
        println!("args error. \n Usage: ycopy src_files dest_dir");
        return;
    }
    
    //let path = current_dir().expect("cur dir is illegal, curiouslly!");
    //println!( "{:?} ", path);

    let param1 = &args[1];
    println!("args0: \"{}\"", param1);

    let mut idx = param1.rfind('\\');
    if idx == None {
        idx = param1.rfind('/');
    }    
    let idx0 = idx.expect("need a path");
    let _in_dir = param1.split_at(idx0).0;

    println!("src dir: \"{}\"", _in_dir);

    let _fmt = &param1[idx0+2..];
    let _out_dir = &args[2];
    
    println!("dest dir: \"{}\"", _out_dir);

    let out_dir2 : &Path = Path::new(_out_dir);
    let in_dir2 : &Path = Path::new(_in_dir);
    visit_dirs(in_dir2, _fmt, out_dir2, &copy_file).expect("visit_dirs error");
}

fn copy_file(dir : &DirEntry, sufifx: &str, dest_dir: &Path) -> () {
    //println!("{}", sufifx);

    let path : PathBuf = dir.path();
    let spath = path.into_os_string();    
    let string = spath.to_str();
    match string {    
        Some(string) => {
            if string.ends_with(sufifx) {
                let mut idx = string.rfind('\\');
                if idx == None {
                    idx = string.rfind('/');
                }    
                let idx0 = idx.expect("path error");                
                let file = string.split_at( idx0+1 ).1;
               
                let dfile = dest_dir.join(file);
                println!("{:?} --> \"{}\"", string, dfile.display());
                fs::copy(string, dfile).expect("fs::copy error");                    
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