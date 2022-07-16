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
use geo::{
    coord, line_string, point, polygon, Coordinate, EuclideanDistance, Intersects, Line,
    LineString, Point, Rect,
};

use rand::Rng;

// --
#[derive(Debug, Copy, Clone)]
enum DrawElem {
    Line(ElemLine),
    Rect(ElemRect),
}

#[derive(Debug, Copy, Clone)]
struct ElemLine {
    vertex: [Coordinate; 2],
    drag_vertex: i32, // 0 from point, 1 end point
}

#[derive(Debug, Copy, Clone)]
struct ElemRect {
    vertex: [Coordinate; 2], // diagonal
    drag_vertex: i32,        // 0 from point, 1 end point
}

impl ElemRect {
    pub fn to_angle_point(&self) -> [Coordinate; 4] {
        let mut tl = coord! {x: self.vertex[0].x as f64, y: self.vertex[0].y as f64};
        let mut tr = coord! {x: 0., y: 0.};
        let mut br = coord! {x: self.vertex[1].x as f64, y: self.vertex[1].y as f64};
        let mut bl = coord! {x: 0., y: 0.};

        if self.vertex[0].x > self.vertex[1].x {
            tl.x = self.vertex[1].x;
            br.x = self.vertex[0].x;
        } else {
            tl.x = self.vertex[0].x;
            br.x = self.vertex[1].x;
        }

        if self.vertex[0].y > self.vertex[1].y {
            tl.y = self.vertex[1].y;
            br.y = self.vertex[0].y;
        } else {
            tl.y = self.vertex[0].y;
            br.y = self.vertex[1].y;
        }

        bl.x = tl.x;
        bl.y = br.y;

        tr.x = br.x;
        tr.y = tl.y;

        [tl, tr, br, bl]
    }
}

// --

#[derive(Clone, Copy)]
enum EventFn {
    ClickLineBtn,
    ClickRectBtn,
    ClickRemoveBtn,
}

#[derive(Clone)]
enum Status {
    CREATING,
    EDIT_MOVING, // default
    EDIT_RESIZING,
    DELETING,
}

struct AppView {
    app: app::App,
    win: window::Window,
    frm: frame::Frame,
    draw_elems: Rc<RefCell<Vec<DrawElem>>>,
    hover_index: Rc<RefCell<i32>>,
    status: Rc<RefCell<Status>>,
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
        let mut remove_btn = button::Button::default().with_label("Remove");
        remove_btn.emit(s, EventFn::ClickRemoveBtn);
        btm_col.end();

        root_col.set_size(&top_col, 500);
        root_col.set_size(&btm_col, 100);
        root_col.end();

        main_win.end();
        main_win.show();

        Self {
            app,
            win: main_win,
            frm,
            draw_elems: Rc::new(RefCell::new(Vec::new())),
            // bk_elem: Rc::new(RefCell::new(None)),
            eventReceiver: receiver,
            hover_index: Rc::new(RefCell::new(0)),
            status: Rc::new(RefCell::new(Status::EDIT_MOVING)),
        }
    }

    fn click_line_btn(&mut self) {
        let line = ElemLine {
            vertex: [coord! {x: 0., y: 0.}, coord! {x: 0., y: 0.}],
            drag_vertex: -1,
        };
        self.draw_elems.borrow_mut().push(DrawElem::Line(line));
        *self.status.borrow_mut() = Status::CREATING;
    }

    fn click_rect_btn(&mut self) {
        self.draw_elems.borrow_mut().push(DrawElem::Rect(ElemRect {
            vertex: [coord! {x: 0., y: 0.}, coord! {x: 0., y: 0.}],
            drag_vertex: -1,
        }));
        *self.status.borrow_mut() = Status::CREATING;
    }

    fn click_remove_btn(&mut self) {
        *self.status.borrow_mut() = Status::DELETING;
    }

    pub fn run(&mut self) {
        self.frm.draw({
            let draw_elems = Rc::clone(&self.draw_elems);
            let hover_index = Rc::clone(&self.hover_index);
            move |frm| {
                for (i, elem) in draw_elems.borrow().iter().enumerate() {
                    match elem {
                        DrawElem::Line(line) => {
                            draw::set_line_style(LineStyle::Solid, 3);

                            if i as i32 == *hover_index.borrow() {
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (line.vertex[0].x - 5.) as i32,
                                    (line.vertex[0].y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (line.vertex[1].x - 5.) as i32,
                                    (line.vertex[1].y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );
                                draw::set_draw_color(Color::DarkRed);
                            } else {
                                draw::set_draw_color(Color::Red);
                            }

                            draw::draw_line(
                                line.vertex[0].x as i32,
                                line.vertex[0].y as i32,
                                line.vertex[1].x as i32,
                                line.vertex[1].y as i32,
                            );
                        }
                        DrawElem::Rect(rect) => {
                            let [tl, tr, br, bl] = rect.to_angle_point();

                            if i as i32 == *hover_index.borrow() {
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (tl.x - 5.) as i32,
                                    (tl.y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (tr.x - 5.) as i32,
                                    (tr.y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (br.x - 5.) as i32,
                                    (br.y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );
                                draw::draw_box(
                                    FrameType::OvalBox,
                                    (bl.x - 5.) as i32,
                                    (bl.y - 5.) as i32,
                                    10,
                                    10,
                                    Color::DarkRed,
                                );

                                draw::set_draw_color(Color::DarkRed);
                            } else {
                                draw::set_draw_color(Color::Red);
                            }

                            draw::set_line_style(LineStyle::Solid, 3);
                            draw::draw_rect(
                                tl.x as i32,
                                tl.y as i32,
                                (tr.x - tl.x) as i32,
                                (bl.y - tl.y) as i32,
                            );
                        }
                    }
                }
            }
        });
        self.frm.handle({
            let draw_elems = Rc::clone(&self.draw_elems);
            let hover_index = Rc::clone(&self.hover_index);
            // let bk_elem = Rc::clone(&self.bk_elem);
            let status = Rc::clone(&self.status);
            let mut tx = 0;
            let mut ty = 0;
            let mut telem: Option<DrawElem> = None;
            move |frm, e| -> bool {
                let (x, y) = app::event_coords();
                let coords_point = point! {
                    x: x as f64,
                    y: y as f64
                };
                match e {
                    Event::Push => {
                        tx = x;
                        ty = y;
                        let idx = *hover_index.borrow();
                        let mut elems = draw_elems.borrow_mut();

                        let mut status = status.borrow_mut();

                        match *status {
                            Status::DELETING => {
                                if idx > -1 {
                                    elems.remove(idx as usize);
                                    *status = Status::EDIT_MOVING;
                                }
                            }
                            Status::EDIT_MOVING => {
                                let elem = elems.get_mut(idx as usize);
                                if let Some(elem) = elem {
                                    match elem {
                                        DrawElem::Line(line) => {
                                            if point! {line.vertex[0]}
                                                .euclidean_distance(&coords_point)
                                                < 10.
                                            {
                                                *status = Status::EDIT_RESIZING;
                                                line.drag_vertex = 0;
                                            } else if point! {line.vertex[1]}
                                                .euclidean_distance(&coords_point)
                                                < 10.
                                            {
                                                *status = Status::EDIT_RESIZING;
                                                line.drag_vertex = 1;
                                            } else {
                                                telem = Some(elem.clone());
                                            }
                                        }
                                        DrawElem::Rect(rect) => {
                                            let [tl, tr, br, bl] = rect.to_angle_point();
                                            if point! {tl}.euclidean_distance(&coords_point) < 10. {
                                                *status = Status::EDIT_RESIZING;
                                                rect.drag_vertex = 0;
                                            } else if point! {tr}.euclidean_distance(&coords_point)
                                                < 10.
                                            {
                                                *status = Status::EDIT_RESIZING;
                                                rect.drag_vertex = 1;
                                            } else if point! {br}.euclidean_distance(&coords_point)
                                                < 10.
                                            {
                                                *status = Status::EDIT_RESIZING;
                                                rect.drag_vertex = 2;
                                            } else if point! {bl}.euclidean_distance(&coords_point)
                                                < 10.
                                            {
                                                *status = Status::EDIT_RESIZING;
                                                rect.drag_vertex = 3;
                                            } else {
                                                telem = Some(elem.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            Status::CREATING => (),
                            Status::EDIT_RESIZING => (),
                        };

                        true
                    }
                    Event::Drag => {
                        match *status.borrow() {
                            Status::CREATING => {
                                let mut elems = draw_elems.borrow_mut();
                                let top = elems.last_mut();
                                if let Some(elem) = top {
                                    match elem {
                                        DrawElem::Line(line) => {
                                            line.vertex[0].x = tx as f64;
                                            line.vertex[0].y = ty as f64;
                                            line.vertex[1].x = x as f64;
                                            line.vertex[1].y = y as f64;
                                        }
                                        DrawElem::Rect(rect) => {
                                            rect.vertex[0].x = tx as f64;
                                            rect.vertex[0].y = ty as f64;
                                            rect.vertex[1].x = x as f64;
                                            rect.vertex[1].y = y as f64;
                                        }
                                    }
                                }
                            }
                            Status::EDIT_MOVING => {
                                let idx = *hover_index.borrow_mut();
                                let mut elems = draw_elems.borrow_mut();
                                let elem = elems.get_mut(idx as usize);
                                if let Some(elem) = elem {
                                    match elem {
                                        DrawElem::Line(line) => {
                                            let x_len = (x - tx) as f64;
                                            let y_len = (y - ty) as f64;
                                            // let telem = bk_elem1.borrow().unwrap();
                                            let telem = telem.unwrap();
                                            if let DrawElem::Line(tline) = telem {
                                                line.vertex[0].x = tline.vertex[0].x + x_len;
                                                line.vertex[0].y = tline.vertex[0].y + y_len;
                                                line.vertex[1].x = tline.vertex[1].x + x_len;
                                                line.vertex[1].y = tline.vertex[1].y + y_len;
                                            }
                                        }
                                        DrawElem::Rect(rect) => {
                                            let x_len = (x - tx) as f64;
                                            let y_len = (y - ty) as f64;
                                            // let telem = bk_elem1.borrow().unwrap();
                                            let telem = telem.unwrap();
                                            if let DrawElem::Rect(trect) = telem {
                                                rect.vertex[0].x = trect.vertex[0].x + x_len;
                                                rect.vertex[0].y = trect.vertex[0].y + y_len;
                                                rect.vertex[1].x = trect.vertex[1].x + x_len;
                                                rect.vertex[1].y = trect.vertex[1].y + y_len;
                                            }
                                        }
                                    }
                                }
                            }
                            Status::EDIT_RESIZING => {
                                let idx = *hover_index.borrow_mut();
                                let mut elems = draw_elems.borrow_mut();
                                let elem = elems.get_mut(idx as usize);
                                if let Some(elem) = elem {
                                    match elem {
                                        DrawElem::Line(line) => match line.drag_vertex {
                                            0 => {
                                                line.vertex[0].x = x as f64;
                                                line.vertex[0].y = y as f64;
                                            }
                                            1 => {
                                                line.vertex[1].x = x as f64;
                                                line.vertex[1].y = y as f64;
                                            }
                                            _ => (),
                                        },
                                        DrawElem::Rect(rect) => match rect.drag_vertex {
                                            0 => {
                                                rect.vertex[0].x = x as f64;
                                                rect.vertex[0].y = y as f64;
                                            }
                                            1 => {
                                                rect.vertex[0].y = y as f64;
                                                rect.vertex[1].x = x as f64;
                                            }
                                            2 => {
                                                rect.vertex[1].x = x as f64;
                                                rect.vertex[1].y = y as f64;
                                            }
                                            3 => {
                                                rect.vertex[0].x = x as f64;
                                                rect.vertex[1].y = y as f64;
                                            }
                                            _ => (),
                                        },
                                    }
                                }
                            }
                            Status::DELETING => (),
                        }

                        frm.redraw();

                        true
                    }
                    Event::Released => {
                        *status.borrow_mut() = Status::EDIT_MOVING;
                        true
                    }
                    Event::Move => {
                        *hover_index.borrow_mut() = -1;
                        let len = draw_elems.borrow_mut().len();
                        for (i, elem) in draw_elems.borrow_mut().iter().rev().enumerate() {
                            match elem {
                                DrawElem::Line(line) => {
                                    let t_line = Line::new(line.vertex[0], line.vertex[1]);

                                    if coords_point.euclidean_distance(&t_line) < 10. {
                                        *hover_index.borrow_mut() = (len - i - 1) as i32;
                                        break;
                                    }
                                }
                                DrawElem::Rect(rect) => {
                                    let t_rect = Rect::new(rect.vertex[0], rect.vertex[1]);

                                    if coords_point.intersects(&t_rect) {
                                        *hover_index.borrow_mut() = (len - i - 1) as i32;
                                        break;
                                    }
                                }
                            }
                        }
                        frm.redraw();
                        true
                    }

                    _ => false,
                }
            }
        });
        while self.app.wait() {
            if let Some(msg) = self.eventReceiver.recv() {
                match msg {
                    EventFn::ClickLineBtn => self.click_line_btn(),
                    EventFn::ClickRectBtn => self.click_rect_btn(),
                    EventFn::ClickRemoveBtn => self.click_remove_btn(),
                }
            }
        }
    }
}

fn main() {
    let mut a = AppView::new();
    a.run();
}
