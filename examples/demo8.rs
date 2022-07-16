use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use fltk::{
    draw::LineStyle,
    enums::{Align, Color, Event, FrameType},
    prelude::*,
    *,
};

#[derive(Clone, Copy)]
enum EventFn {
    ClickLineBtn,
    ClickRectBtn,
}

struct AppView {
    app: app::App,
    win: window::Window,
    frm: frame::Frame,
    draw_frms: Vec<frame::Frame>,
    eventReceiver: app::Receiver<EventFn>,
}

impl AppView {
    pub fn new() -> Self {
        let app = app::App::default();
        let (s, receiver) = app::channel();
        let mut main_win = window::Window::default().with_size(500, 600);

        let mut root_col = group::Flex::new(0, 0, 500, 600, None).column();

        let top_col = group::Flex::default().row();
        let mut frm = frame::Frame::default();
        frm.set_frame(FrameType::FlatBox);
        frm.set_color(Color::White);
        top_col.end();

        let btm_col = group::Flex::default().row();
        let mut line_btn = button::Button::default().with_label("Line");
        line_btn.emit(s, EventFn::ClickLineBtn);
        let mut rect_btn = button::Button::default().with_label("Rect");
        rect_btn.emit(s, EventFn::ClickRectBtn);
        btm_col.end();

        root_col.set_size(&top_col, 500);
        root_col.set_size(&btm_col, 100);
        root_col.end();

        main_win.end();
        main_win.show();
        // main_win.resizable(&frm);

        Self {
            app,
            win: main_win,
            frm,
            draw_frms: Vec::new(),
            eventReceiver: receiver,
        }
    }

    fn click_line_btn(&mut self) {
        let mut draw_frm = frame::Frame::new(0, 0, 500, 500, None);
        draw_frm.set_frame(FrameType::FlatBox);
        draw_frm.set_color(Color::Yellow);
        draw_frm.set_label(self.draw_frms.len().to_string().as_str());

        let from_x = Rc::new(RefCell::new(0));
        let from_y = Rc::new(RefCell::new(0));
        let end_x = Rc::new(RefCell::new(0));
        let end_y = Rc::new(RefCell::new(0));

        draw_frm.handle({
            let from_x = Rc::clone(&from_x);
            let from_y = Rc::clone(&from_y);
            let end_x = Rc::clone(&end_x);
            let end_y = Rc::clone(&end_y);
            let mut win = self.win.clone();
            let mut root_frm = self.frm.clone();
            move |frm, e| {
                let (x, y) = app::event_coords();
                match e {
                    Event::Push => {
                        *from_x.borrow_mut() = x;
                        *from_y.borrow_mut() = y;

                        true
                    }
                    Event::Drag => {
                        *end_x.borrow_mut() = x;
                        *end_y.borrow_mut() = y;
                        frm.set_pos(0, 0);
                        frm.set_size(500, 500);
                        frm.redraw();
                        win.redraw();
                        true
                    }
                    Event::Released => {
                        let (mut fx, mut fy, mut ex, mut ey) = (
                            *from_x.borrow(),
                            *from_y.borrow(),
                            *end_x.borrow(),
                            *end_y.borrow(),
                        );

                        let mut t = 0;
                        if fx > ex {
                            t = fx;
                            fx = ex;
                            ex = t;
                        }

                        if fy > ey {
                            t = fy;
                            fy = ey;
                            ey = t;
                        }

                        if ex > fx && ey > fy {
                            frm.set_pos(fx, fy);
                            frm.set_size(ex - fx, ey - fy);
                        } else if ex > fx && ey < fy {
                            frm.set_pos(fx, ey);
                            frm.set_size(ex - fx, fy - ey);
                        }

                        frm.redraw();
                        win.redraw();
                        true
                    }
                    Event::Move => {
                        // println!("{:?}", frm.label());
                        true
                    }
                    Event::Leave => {
                        println!("{:?}", frm.label());
                        true
                    }
                    _ => {
                        // println!("{:?}", e);
                        false
                    }
                }
            }
        });

        draw_frm.draw({
            let from_x = Rc::clone(&from_x);
            let from_y = Rc::clone(&from_y);
            let end_x = Rc::clone(&end_x);
            let end_y = Rc::clone(&end_y);
            move |frm| {
                draw::set_line_style(LineStyle::Solid, 3);
                draw::set_draw_color(Color::Red);
                draw::draw_line(
                    *from_x.borrow(),
                    *from_y.borrow(),
                    *end_x.borrow(),
                    *end_y.borrow(),
                );
            }
        });

        self.win.add(&draw_frm);
        self.win.redraw();
        self.draw_frms.push(draw_frm);
    }

    fn click_rect_btn(&mut self) {}

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(msg) = self.eventReceiver.recv() {
                match msg {
                    EventFn::ClickLineBtn => self.click_line_btn(),
                    EventFn::ClickRectBtn => self.click_rect_btn(),
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    let mut a = AppView::new();
    a.run();
}
