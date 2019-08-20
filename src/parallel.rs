
extern crate time;
extern crate rand;
 
use std::thread;
use time::*;
use rand::Rng;
fn main() {
    let start = time::now();
 
    let handles: Vec<_> = (0..8)
        .map(|_| {
            thread::spawn(|| {
                // 防止被编译器优化，随机化初始值
                let mut rng = rand::thread_rng();
                let mut x = rng.gen::<i32>();
                let num = 5_000;    //_000;
                for i in 0..num {
                    // 防止被编译器优化，随机化操作
                    if (x + i) % 2 == 0 {
                        x += i;
                    } else {
                        x -= i;
                    }
                }
                x
            })
        })
        .collect();
    for h in handles {
        println!("Thread finished with count={}",
                 h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
 
    let end = time::now();
    let duration = end - start;
 
    println!("耗时:{}", duration);
}