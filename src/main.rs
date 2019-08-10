/*

*!
*@
*#
*%
*

*/

use std::env::*;
use std::ptr;

/*use std::path;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
*/
use std::collections::HashMap;

//宏的使用
macro_rules! hashmap {
    ( $($key:expr => $val : expr), * ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            //$(map.insert($key, $val);)*
            $(map.insert($key, $val));*;
            map
        }        
    }
}

fn process_or_insert(map: &mut HashMap<String, String>, key : String, val: String)
{
    match map.get_mut(&key)
    {
        Some(value) => println!("{}", value ),
        None => {
            map.insert(key, val);
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

    let counts = hashmap!['a' => 0, 'b' => 1, 'c' => 3];
    println!("{:?}", counts);


    let mut map = HashMap::<String, String>::new();
    let mut key = &String::from("key");
    let mut val = &String::from("val");
    process_or_insert(&mut map, key.to_string(), val.to_string());

    for i in map{
        println!("{} {}", i.0, i.1 );
    }

    println!("{} {}", key, val);    
    unsafe { ptr::swap( &mut key, &mut val ); }
    println!("{} {} ", key, val);

    /*
    let mut path = env::home_dir().unwrap();
    let in_dir = args[1];
    let idx = in_dir.find(".");
    let (in_dir, suffix) = in_dir.split_at(idx);
    let out_dir = args[2];

    for entry in fs::read_dir(in_dir)? {
        let dir = entry?;
        println!("{}", dir);
    }
*/
}