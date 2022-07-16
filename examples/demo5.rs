use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

struct MyCustomButton {
    inner: widget::Widget,
    num_clicks: Rc<RefCell<i32>>,
}

impl MyCustomButton {
    // our constructor
    pub fn new(radius: i32, label: &str) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(radius * 2, radius * 2)
            .with_label(label)
            .center_of_parent();
        inner.set_frame(enums::FrameType::OFlatBox);
        let num_clicks = 0;
        let num_clicks = Rc::from(RefCell::from(num_clicks));
        let clicks = num_clicks.clone();
        inner.draw(|i| {
            // we need a draw implementation
            draw::draw_box(i.frame(), i.x(), i.y(), i.w(), i.h(), i.color());
            draw::set_draw_color(enums::Color::Black); // for the text
            draw::set_font(enums::Font::Helvetica, app::font_size());
            draw::draw_text2(&i.label(), i.x(), i.y(), i.w(), i.h(), i.align());
        });
        inner.handle(move |i, ev| match ev {
            enums::Event::Push => {
                *clicks.borrow_mut() += 1; // increment num_clicks
                i.do_callback(); // do the callback which we'll set using set_callback().
                true
            }
            _ => false,
        });
        Self { inner, num_clicks }
    }

    // get the times our button was clicked
    pub fn num_clicks(&self) -> i32 {
        *self.num_clicks.borrow()
    }
}

// Extend widget::Widget via the member `inner` and add other initializers and constructors
widget_extends!(MyCustomButton, widget::Widget, inner);

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::background(255, 255, 255); // make the background white
    let mut wind = window::Window::new(100, 100, 400, 300, "Hello from rust");
    let mut btn = MyCustomButton::new(50, "Click");
    btn.set_color(enums::Color::Cyan);
    btn.set_callback(|_| println!("Clicked"));
    let mut btn2 = MyCustomButton::new(50, "Click2");
    btn2.set_color(enums::Color::Red);
    btn2.set_pos(0, 0);
    wind.end();
    wind.show();

    app.run().unwrap();

    // print the number our button was clicked on exit
    println!("Our button was clicked {} times", btn.num_clicks());
}
