use std::thread;
use std::sync::mpsc::channel;
use std::cell::RefCell;

fn main()
{
    let x = RefCell::new(1);

    let mut mutable_borrow = x.borrow_mut();
    *mutable_borrow = 2;

    drop(mutable_borrow); // relinquish the mutable borrow on this slot

    let borrow = x.borrow();
    println!("{}", *borrow);

    /////////////////////////////////////////
    
    let (tx, rx) = channel();    
    for i in 1..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    drop(tx);

    while let Ok(r) = rx.recv() {
        println!( "recv {}", r );
    }
}