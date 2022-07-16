use std::rc::Rc;

use fltk::{
    app::{self},
    button,
    draw::{self, draw_line, set_draw_color, set_line_style, LineStyle, Offscreen},
    enums::{self, Color, Event, EventState, FrameType},
    frame, menu,
    prelude::{GroupExt, ImageExt, MenuExt, SurfaceDevice, WidgetBase, WidgetExt},
    surface, window,
};

fn main() {
    let a = app::App::default();

    let mut win = window::Window::new(0, 0, 500, 600, "Draw");

    let mut menu = menu::MenuButton::default().with_type(menu::MenuButtonType::Popup123);

    let mut f1 = frame::Frame::new(0, 0, 500, 500, "");
    f1.set_frame(FrameType::FlatBox);
    f1.set_color(Color::White);

    let mut f2 = frame::Frame::new(0, 0, 400, 400, "");
    f2.set_frame(FrameType::FlatBox);
    f2.set_color(Color::Blue);

    let mut f3 = frame::Frame::new(0, 0, 200, 200, None);
    f3.set_frame(FrameType::FlatBox);
    f3.set_color(Color::Yellow);
    win.end();
    win.show();

    menu.add("UP", EventState::None, menu::MenuFlag::Normal, {
        let mut win = win.clone();
        let mut f2 = f2.clone();
        move |eb| {
            let idx = win.find(&f2) + 1;
            win.remove(&f2);
            win.insert(&f2, idx);
            win.redraw();
        }
    });

    menu.add("DOWN", EventState::None, menu::MenuFlag::Normal, {
        let mut win = win.clone();
        let mut f2 = f2.clone();
        move |eb| {
            let mut idx = win.find(&f2) - 1;
            idx = if idx == -1 { 0 } else { idx };
            win.remove(&f2);
            win.insert(&f2, idx);
            win.redraw();
        }
    });

    f2.handle({
        move |f2, e| match e {
            Event::Push => {
                menu.popup();
                true
            }
            _ => false,
        }
    });

    a.run().unwrap();
}
