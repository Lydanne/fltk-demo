enum Method {
    Cb(Box<dyn Fn(i32) -> i32>),
}

fn get_closure() -> Method {
    let my_closure = |i: i32| i * i;
    Method::Cb(Box::new(my_closure))
}

fn callback<F: FnMut() + 'static>(cb: F) -> F {
    return cb;
}

fn main() {
    let my_closure = get_closure();
    let mut call = callback(|| println!("hello"));
    call();
    if let Method::Cb(cb) = my_closure {
        println!("{:?}", cb(22));
    }
}
