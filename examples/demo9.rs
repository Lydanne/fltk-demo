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

// --

trait Elem {
    fn draw(&self, hover: bool);
    fn get_vertex(&self) -> Vec<Coordinate<f64>>;
    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32);
    fn hover_condition(&self, mouse_point: Point) -> bool;
}

#[derive(Debug, Copy, Clone)]
struct ElemLine {
    coords: [Coordinate; 2],
}

impl Elem for ElemLine {
    fn draw(&self, hover: bool) {
        let line = self;
        draw::set_line_style(LineStyle::Solid, 3);
        if hover {
            draw::draw_box(
                FrameType::OvalBox,
                (line.coords[0].x - 5.) as i32,
                (line.coords[0].y - 5.) as i32,
                10,
                10,
                Color::DarkRed,
            );
            draw::draw_box(
                FrameType::OvalBox,
                (line.coords[1].x - 5.) as i32,
                (line.coords[1].y - 5.) as i32,
                10,
                10,
                Color::DarkRed,
            );
            draw::set_draw_color(Color::DarkRed);
        } else {
            draw::set_draw_color(Color::Red);
        }

        draw::draw_line(
            line.coords[0].x as i32,
            line.coords[0].y as i32,
            line.coords[1].x as i32,
            line.coords[1].y as i32,
        );
    }

    fn get_vertex(&self) -> Vec<Coordinate> {
        Vec::from(self.coords)
    }

    fn hover_condition(&self, mouse_point: Point) -> bool {
        let t_line = Line::new(self.coords[0], self.coords[1]);
        mouse_point.euclidean_distance(&t_line) < 10.
    }

    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        self.coords[0] = from_coord;
        self.coords[1] = end_coord;
    }

    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let x_dif = end_coord.x - from_coord.x;
        let y_dif = end_coord.y - from_coord.y;

        self.coords[0].x = self.coords[0].x + x_dif;
        self.coords[0].y = self.coords[0].y + y_dif;
        self.coords[1].x = self.coords[1].x + x_dif;
        self.coords[1].y = self.coords[1].y + y_dif;
    }

    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32) {
        match drag_vertex {
            0 => {
                self.coords[0] = end_coord;
            }
            1 => {
                self.coords[1] = end_coord;
            }
            _ => (),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ElemRect {
    // tl_coord: Coordinate, // top left coord
    // width: i32,
    // height: i32,
    coords: [Coordinate; 2], // diagonal
}

impl Elem for ElemRect {
    fn draw(&self, hover: bool) {
        let rect = self;
        let vec = rect.get_vertex();
        let [tl, tr, br, bl] = [vec[0], vec[1], vec[2], vec[3]];

        if hover {
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
        draw::draw_line(
            rect.coords[0].x as i32,
            rect.coords[0].y as i32,
            rect.coords[1].x as i32,
            rect.coords[1].y as i32,
        )
    }

    fn get_vertex(&self) -> Vec<Coordinate> {
        let mut tl = coord! {x: self.coords[0].x, y: self.coords[0].y};
        let mut tr = coord! {x: 0., y: 0.};
        let mut br = coord! {x: self.coords[1].x, y: self.coords[1].y};
        let mut bl = coord! {x: 0., y: 0.};

        let mut t = 0.;
        if tl.x > br.x {
            t = tl.x;
            tl.x = br.x;
            br.x = t;
        }
        if tl.y > br.y {
            t = tl.y;
            tl.y = br.y;
            br.y = t;
        }

        bl.x = tl.x;
        bl.y = br.y;

        tr.x = br.x;
        tr.y = tl.y;

        Vec::from([tl, tr, br, bl])
    }

    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        self.coords[0] = from_coord;
        self.coords[1] = end_coord;
        // if rect.vertex[0].x > rect.vertex[1].x {
        //     rect.vertex[1].x = tx as f64;
        //     rect.vertex[0].x = x as f64;
        // }

        // if rect.vertex[0].y > rect.vertex[1].y {
        //     rect.vertex[1].y = ty as f64;
        //     rect.vertex[0].y = y as f64;
        // }
    }

    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let x_dif = end_coord.x - from_coord.x;
        let y_dif = end_coord.y - from_coord.y;

        self.coords[0].x = self.coords[0].x + x_dif;
        self.coords[0].y = self.coords[0].y + y_dif;
        self.coords[1].x = self.coords[1].x + x_dif;
        self.coords[1].y = self.coords[1].y + y_dif;
    }

    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32) {
        match drag_vertex {
            0 => self.coords[0] = end_coord,
            1 => {
                self.coords[0].y = end_coord.y;
                self.coords[1].x = end_coord.x;
            }
            2 => self.coords[1] = end_coord,
            3 => {
                self.coords[0].x = end_coord.x;
                self.coords[1].y = end_coord.y;
            }
            _ => (),
        }
    }

    fn hover_condition(&self, mouse_point: Point) -> bool {
        Rect::new(self.coords[0], self.coords[1]).intersects(&mouse_point)
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
    draw_elems: Rc<RefCell<Vec<Box<dyn Elem>>>>,
    hover_index: Rc<RefCell<i32>>,
    drag_vertex: Rc<RefCell<i32>>,
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
            eventReceiver: receiver,
            hover_index: Rc::new(RefCell::new(0)),
            drag_vertex: Rc::new(RefCell::new(0)),
            status: Rc::new(RefCell::new(Status::EDIT_MOVING)),
        }
    }

    fn click_line_btn(&mut self) {
        let line = ElemLine {
            coords: [coord! {x: 0., y: 0.}, coord! {x: 0., y: 0.}],
        };
        self.draw_elems.borrow_mut().push(Box::new(line));
        *self.status.borrow_mut() = Status::CREATING;
    }

    fn click_rect_btn(&mut self) {
        self.draw_elems.borrow_mut().push(Box::new(ElemRect {
            coords: [coord! {x: 0., y: 0.}, coord! {x: 0., y: 0.}],
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
                for (i, elem) in draw_elems.borrow_mut().iter_mut().enumerate() {
                    elem.draw(i as i32 == *hover_index.borrow());
                }
            }
        });
        self.frm.handle({
            let draw_elems = Rc::clone(&self.draw_elems);
            let hover_index = Rc::clone(&self.hover_index);
            let drag_vertex = Rc::clone(&self.drag_vertex);
            let status = Rc::clone(&self.status);
            let mut tx = 0;
            let mut ty = 0;
            move |frm, e| -> bool {
                let (x, y) = app::event_coords();
                let mouse_point = point! {
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
                        let elem = elems.get_mut(idx as usize);

                        if let Some(elem) = elem {
                            match *status {
                                Status::DELETING => {
                                    if idx > -1 {
                                        elems.remove(idx as usize);
                                        *status = Status::EDIT_MOVING;
                                    }
                                }
                                Status::EDIT_MOVING => {
                                    let vertex = elem.get_vertex();

                                    for (i, coord) in vertex.iter().enumerate() {
                                        let point = Point::new(coord.x, coord.y);
                                        if mouse_point.euclidean_distance(&point) < 10. {
                                            *status = Status::EDIT_RESIZING;
                                            *drag_vertex.borrow_mut() = i as i32;
                                            *status = Status::EDIT_RESIZING;
                                        }
                                    }
                                }
                                Status::CREATING => (),
                                Status::EDIT_RESIZING => (),
                            };
                        }

                        true
                    }
                    Event::Drag => {
                        match *status.borrow() {
                            Status::CREATING => {
                                let mut elems = draw_elems.borrow_mut();
                                let top = elems.last_mut();
                                if let Some(elem) = top {
                                    elem.creating(
                                        coord! {x: tx as f64, y: ty as f64},
                                        coord! {x: x  as f64, y: y as f64},
                                    )
                                }
                            }
                            Status::EDIT_MOVING => {
                                let idx = *hover_index.borrow_mut();
                                let mut elems = draw_elems.borrow_mut();
                                let elem = elems.get_mut(idx as usize);
                                if let Some(elem) = elem {
                                    elem.edit_moving(
                                        coord! {x: tx as f64, y: ty as f64},
                                        coord! {x: x  as f64, y: y as f64},
                                    );
                                    tx = x;
                                    ty = y;
                                }
                            }
                            Status::EDIT_RESIZING => {
                                let idx = *hover_index.borrow_mut();
                                let mut elems = draw_elems.borrow_mut();
                                let elem = elems.get_mut(idx as usize);
                                if let Some(elem) = elem {
                                    elem.edit_resizing(
                                        coord! {x: tx as f64, y: ty as f64},
                                        coord! {x: x  as f64, y: y as f64},
                                        *drag_vertex.borrow(),
                                    );
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
                            if elem.hover_condition(mouse_point) {
                                *hover_index.borrow_mut() = (len - i - 1) as i32;
                                break;
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
