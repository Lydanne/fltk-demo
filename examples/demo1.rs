use std::{cell::RefCell, rc::Rc};

use fltk::{
    app::{self},
    button,
    draw::{self, draw_line, set_draw_color, set_line_style, LineStyle, Offscreen},
    enums::{self, Color, Cursor, Event, FrameType},
    frame,
    prelude::{GroupExt, ImageExt, SurfaceDevice, WidgetBase, WidgetExt},
    surface, window,
};
use tray_item::TrayItem;

fn main() {
    // create_tray_bar().inner_mut().display();
    create_win();
}

fn create_win() {
    let a = app::App::default();

    let mut win = window::Window::new(0, 0, 500, 600, "Draw");

    let mut frm = frame::Frame::new(0, 0, 500, 500, "");
    frm.set_frame(FrameType::FreeBoxType);
    frm.set_color(Color::White);

    let mut draw_part = DrawPart::new(frm.clone());
    draw_part.draw_line(Pos(100, 100), Pos(200, 200));
    // let mut draw_part1 = DrawPart::new(frm.clone());
    // draw_part1.draw_line(Pos(300, 200), Pos(300, 300));

    let mut draw_btn = button::Button::new(100, 530, 100, 50, "Draw");
    let mut eraser_btn = button::Button::new(200, 530, 100, 50, "Eraser");
    let mut save_btn = button::Button::new(300, 530, 100, 50, "Save");

    win.end();
    win.show();

    let (se, re) = app::channel::<u8>(); // 0 => draw, 1 => eraser

    draw_btn.set_callback({
        move |btn| {
            se.send(0);
        }
    });

    eraser_btn.set_callback({
        move |btn| {
            se.send(1);
        }
    });

    let offs = Offscreen::new(500, 500).unwrap();

    let offs = Rc::new(offs);

    save_btn.set_callback({
        let mut win = win.clone();
        let frm = frm.clone();
        let offs = offs.clone();
        move |btn| {
            let sur = surface::ImageSurface::new(frm.w(), frm.h(), false);
            surface::ImageSurface::push_current(&sur);
            draw::set_draw_color(enums::Color::White);
            draw::draw_rectf(0, 0, frm.w(), frm.h());
            sur.draw(&frm, 0, 0);
            let img = sur.image().unwrap();
            surface::ImageSurface::pop_current();
            let mut imgbuf: image::RgbImage = image::ImageBuffer::new(frm.w() as _, frm.h() as _); // this is from the image crate
            imgbuf.copy_from_slice(&img.to_rgb_data());
            imgbuf.save("image.jpg").unwrap();
            win.hide();
        }
    });

    // frm.handle({
    //     let offs = offs.clone();
    //     let mut mode = 0;
    //     let (mut nx, mut ny) = (0, 0);

    //     move |frm, e| {
    //         let (x, y) = app::event_coords();

    //         return match e {
    //             // Event::Push => {
    //             //     if let Some(v) = re.recv() {
    //             //         mode = v;
    //             //     };
    //             //     nx = x;
    //             //     ny = y;
    //             //     true
    //             // }
    //             Event::Drag => {
    //                 // offs.begin();
    //                 // set_draw_color(if mode == 0 { Color::Red } else { Color::White });
    //                 // set_line_style(LineStyle::Solid, 3);
    //                 // draw_line(nx, ny, x, y);
    //                 // nx = x;
    //                 // ny = y;
    //                 // offs.end();
    //                 // frm.redraw();
    //                 // set_line_style(LineStyle::Solid, 0);
    //                 println!("move");
    //                 true
    //             }
    //             _ => false,
    //         };
    //     }
    // });
    frm.draw({
        let mut win = win.clone();
        let offs = offs.clone();
        move |frm| {
            // offs.copy(0, 0, 500, 500, 0, 0);
            // draw_part.flush();
            // draw_part1.flush();
            // win.redraw();
        }
    });

    a.run().unwrap();
}

fn create_tray_bar() -> TrayItem {
    let mut bar = TrayItem::new("Foo", "").unwrap();

    bar.add_menu_item(
        "Open",
        Box::new({
            move || {
                create_win();
            }
        }),
    )
    .unwrap();

    bar.add_menu_item("Quit", Box::new(|| std::process::exit(0)))
        .unwrap();
    bar
}

#[derive(Default)]
struct DrawPartManager {
    frmVec: Vec<frame::Frame>,
}

impl DrawPartManager {
    pub fn new() -> Self {
        let mut frm = frame::Frame::default();
        frm.set_frame(FrameType::FlatBox);
        let mut frmVec = vec![frm];
        DrawPartManager { frmVec }
    }
}

struct Pos(i32, i32);

struct Size(i32, i32);

enum Status {
    None,
    Select,
    DragLeft,
    DragRight,
}

struct DrawPart {
    frm: frame::Frame,
    status: Rc<RefCell<Status>>,
}

impl DrawPart {
    pub fn new(mut parent_frm: frame::Frame) -> Self {
        let mut frm = frame::Frame::default();
        let (mut inner_x, mut inner_y) = (0, 0);
        let (mut end_x, mut end_y) = (0, 0);
        let mut status = Rc::new(RefCell::new(Status::None));
        frm.handle({
            let mut parent_frm = parent_frm.clone();
            let status = status.clone();
            move |frm, ev| {
                let (x, y) = app::event_coords();
                match ev {
                    Event::Push => {
                        (inner_x, inner_y) = (x - frm.x(), y - frm.y());
                        (end_x, end_y) = (x + frm.w(), y + frm.h());
                        frm.set_frame(FrameType::BorderFrame);
                        frm.redraw();

                        if inner_x < frm.w() + 10
                            && inner_x > frm.w() - 10
                            && inner_y < frm.h() + 10
                            && inner_y > frm.h() - 10
                        {
                            *status.borrow_mut() = Status::DragRight;
                            draw::set_cursor(Cursor::Cross);
                        } else if inner_x < 10 && inner_x > -10 && inner_y < 10 && inner_y > -10 {
                            *status.borrow_mut() = Status::DragLeft;
                            draw::set_cursor(Cursor::Cross);
                        } else {
                            draw::set_cursor(Cursor::Default);
                            *status.borrow_mut() = Status::Select;
                        }
                        true
                    }
                    Event::Drag => {
                        let status = status.borrow();
                        if let Status::DragRight = *status {
                            frm.set_size(x - frm.x(), y - frm.y());
                        } else if let Status::DragLeft = *status {
                            frm.set_pos(x, y);
                            frm.set_size(end_x - x, end_y - y);
                        } else {
                            frm.set_pos(x - inner_x, y - inner_y);
                        }
                        frm.redraw();
                        parent_frm.redraw();

                        true
                    }
                    Event::Move => {
                        (inner_x, inner_y) = (x - frm.x(), y - frm.y());

                        if inner_x < frm.w() + 10
                            && inner_x > frm.w() - 10
                            && inner_y < frm.h() + 10
                            && inner_y > frm.h() - 10
                        {
                            *status.borrow_mut() = Status::DragRight;
                            draw::set_cursor(Cursor::Cross);
                        } else if (inner_x < 10 && inner_x > -10 && inner_y < 10 && inner_y > -10) {
                            *status.borrow_mut() = Status::DragRight;
                            draw::set_cursor(Cursor::Cross);
                        } else {
                            draw::set_cursor(Cursor::Default);
                            *status.borrow_mut() = Status::Select;
                        }
                        true
                    }
                    _ => false,
                }
            }
        });
        parent_frm.handle({
            let mut frm = frm.clone();
            let select = status.clone();
            move |parent_frm, e| match e {
                Event::Push => {
                    frm.set_frame(FrameType::NoBox);
                    frm.redraw();
                    *select.borrow_mut() = Status::None;
                    parent_frm.redraw();
                    true
                }
                Event::Move => {
                    draw::set_cursor(Cursor::Default);
                    true
                }
                _ => false,
            }
        });
        DrawPart { frm, status }
    }

    pub fn draw_line(&mut self, from: Pos, to: Pos) {
        let (mut w, mut h) = ((from.0 - to.0).abs(), (from.1 - to.1).abs());
        if w == 0 {
            w = 3;
        }
        if h == 0 {
            h = 3;
        }

        self.frm.set_pos(from.0, from.1);
        self.frm.set_size(w, h);
        self.frm.draw({
            let mut select = Rc::clone(&self.status);
            move |frm| {
                let (sx, sy, ex, ey) = (frm.x(), frm.y(), frm.x() + frm.w(), frm.y() + frm.h());
                draw::set_cursor(Cursor::Default);
                draw::set_draw_color(Color::Red);
                draw::set_line_style(LineStyle::Solid, 3);
                draw::draw_line(sx, sy, ex, ey);
                match *select.borrow() {
                    Status::None => {
                        draw::set_cursor(Cursor::Default);
                    }
                    Status::Select => {
                        draw::set_draw_color(Color::Black);
                        draw::draw_circle((sx + 2) as f64, (sy + 2) as f64, 3.0);
                        draw::draw_circle((ex - 2) as f64, (ey - 2) as f64, 3.0);
                    }
                    _ => {
                        draw::set_draw_color(Color::Black);
                        draw::draw_circle((sx + 2) as f64, (sy + 2) as f64, 3.0);
                        draw::draw_circle((ex - 2) as f64, (ey - 2) as f64, 3.0);
                        draw::set_cursor(Cursor::Cross);
                    }
                }
            }
        });
    }

    pub fn flush(&mut self) {
        self.frm.redraw();
    }
}
