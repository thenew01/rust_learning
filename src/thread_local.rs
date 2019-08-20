use std::cell::RefCell;
use std::thread;

fn main()
{
    thread_local!{
        static foo : RefCell<u32> = RefCell::new(1);
    }
    foo.with(|f| {
        println!( "main thread value1 {:?}", *f.borrow());
        *f.borrow_mut() = 2;
        println!("main thread value2 {:?}", *f.borrow());
    });


    let t = thread::spawn(move || {
        foo.with(|f|{
            println!(" child thread value1 {:?}", *f.borrow());
            *f.borrow_mut() = 2;
            println!(" child thread value2 {:?}", *f.borrow());
        });
    });

    t.join().ok();

    foo.with(|f|{
        println!("main thread value2 {:?}", *f.borrow());
    });
}