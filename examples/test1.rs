use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();

    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default()
        .with_size(100, 100)
        .column()
        .center_of_parent();
    let mut show = button::Button::default().with_label("Show");
    let label = frame::Frame::default().with_label("Enter age");
    let input = input::IntInput::default();
    let mut btn = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    show.set_callback({
        let mut input = input.clone();
        move |btn| {
            input.take_focus();
        }
    });

    btn.set_callback(move |btn| {
        println!("your age is {}", input.value());
    });

    a.run().unwrap();
}
