use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::thread;

trait FnMove {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnMove for F {
    fn call(self: Box<Self>) {
        (*self)()
    }
}

fn main() {
    let val = AtomicUsize::new(0);
    let mut guards = vec![];
    for _ in 0..8 {
        let closure: Box<dyn FnMove + Send> = unsafe {
            std::mem::transmute(Box::new(|| {
                let v = val.fetch_add(1, AtomicOrdering::SeqCst);
                println!("{:?}", v);
            }) as Box<dyn FnMove>)
        };

        guards.push(thread::spawn(move || closure.call()));
    }

    for guard in guards {
        guard.join().unwrap();
    }
    println!("over");
}
