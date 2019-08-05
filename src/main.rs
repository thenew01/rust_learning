/*

*!
*@
*#
*%
*

*/

use std::env::*;
/*use std::path;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
*/
use std::collections::HashMap;

fn process_or_insert(map: &mut HashMap<String, String>, key : String)
{
    match map.get_mut(&key)
    {
        Some(value) => println!("{}", value ),
        None => {
            map.insert(key, String::new());
        }
    }

}

fn main() {
    println!("Hello, world!");
    let args : Vec<String> = args().collect();
    if args.len() != 3 
    {
        println!("para error");
        //return;
    }


    let mut map = HashMap::<String, String>::new();
    let key = String::from("abc");
    process_or_insert(&mut map, key);

    for i in  map{
        println!("{} {}", i.0, i.1 );
    }

/*
    //let mut path = env::home_dir().unwrap();
    let in_dir = args[1];
    let idx = in_dir.find(".");
    let (in_dir, suffix) = in_dir.split_at(idx);
    let out_dir = args[2];

    for entry in fs::read_dir(in_dir)? {
        let dir = entry?;
        println!("{:?}", dir.path());

    }
*/
}
