/*

*!
*@
*#
*%
*

*/

use std::ptr;
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

macro_rules! _max {
    ($a : tt, $b: tt) => {
        if $a >= $b { $a } else { $b }
    };
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
   
    let a = 2;
    let b = 3;
    let c = _max!(a, b);

    println!( "max({}, {}) is {}", a, b, c);

//   Result, Some, Option

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


}