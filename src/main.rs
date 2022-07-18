use std::{ops::Deref, rc::Rc};

use core_graphics::{display::CGDisplay, image::CGImage};
use fltk::{
    app::{self, App, Scheme},
    draw::{self, Offscreen},
    enums::{self, *},
    frame, menu,
    prelude::*,
    surface, window,
};

use image::{self, imageops, EncodableLayout};
use tray_item::TrayItem;

fn create_tray_bar() -> TrayItem {
    let mut bar = TrayItem::new("Foo", "").unwrap();

    bar.add_menu_item(
        "截屏",
        Box::new({
            move || {
                capture_screen_win();
            }
        }),
    )
    .unwrap();

    bar.add_menu_item("Quit", Box::new(|| std::process::exit(0)))
        .unwrap();
    bar
}

fn capture_screen_cg_image(id: u32) -> CGImage {
    let cg_display = CGDisplay::new(id);
    let cg_image = cg_display.image().unwrap();
    cg_image
}

fn bgra_to_rgba(mut bgra: Vec<u8>) -> Vec<u8> {
    for i in (0..bgra.len()).step_by(4) {
        let b = bgra[i];
        let r = bgra[i + 2];

        bgra[i] = r;
        bgra[i + 2] = b;
        bgra[i + 3] = 255;
    }

    bgra
}

fn capture_screen_win() {
    let a = app::App::default().with_scheme(Scheme::Gtk);
    let (x, y, sw, sh) = app::screen_xywh(0);

    let mut win = window::Window::new(x, y, sw, sh, None);
    let mut frm = frame::Frame::new(x, y, sw, sh, None);
    let mut sel_frm = frame::Frame::new(0, 0, 0, 0, None);
    sel_frm.set_frame(FrameType::BorderFrame);
    let mut menu = menu::MenuButton::default().with_type(menu::MenuButtonType::Popup123);
    win.end();
    win.show();
    win.set_border(false);
    win.set_on_top();
    win.resize(x, y, sw, sh);

    // let screen_image = create_screen_image();
    // let image = PngImage::from_data(screen_image.buffer());
    let cg_image = capture_screen_cg_image(1);

    let cg_w = cg_image.width() as u32;
    let cg_h = cg_image.height() as u32;

    let bgra = Vec::from(cg_image.data().as_bytes());

    let rbga = bgra_to_rgba(bgra);

    let mut img =
        fltk::image::RgbImage::new(rbga.as_bytes(), cg_w as i32, cg_h as i32, ColorDepth::Rgba8)
            .unwrap();
    img.scale(sw as i32, sh as i32, true, true);
    frm.set_image(Some(img));

    menu.add("关闭", EventState::None, menu::MenuFlag::Normal, {
        let mut win = win.clone();
        // let mut offs = offs.clone();
        move |eb| {
            win.hide();
        }
    });
    menu.add("保存", EventState::None, menu::MenuFlag::Normal, {
        let mut win = win.clone();
        let mut frm = frm.clone();
        let mut sel_frm = sel_frm.clone();
        move |eb| {
            // println!("{:?}",(sel_frm.x(), sel_frm.y(), sel_frm.w(), sel_frm.h()));
            let sur = surface::ImageSurface::new(frm.w(), frm.h(), false);
            surface::ImageSurface::push_current(&sur);
            draw::set_draw_color(enums::Color::White);
            draw::draw_rectf(0, 0, frm.w(), frm.h());
            sur.draw(&frm, 0, 0);
            let img = sur.image().unwrap();
            surface::ImageSurface::pop_current();
            let mut imgbuf: image::RgbImage = image::ImageBuffer::new(frm.w() as _, frm.h() as _);
            imgbuf.copy_from_slice(&img.to_rgb_data());
            let subimg = imageops::crop(
                &mut imgbuf,
                sel_frm.x() as u32,
                sel_frm.y() as u32,
                sel_frm.w() as u32,
                sel_frm.h() as u32,
            );
            subimg.to_image().save("image.jpg").unwrap();
            win.hide();
        }
    });

    frm.handle({
        // let offs = offs.clone();
        let mut sel_frm = sel_frm.clone();
        let mut tx: i32 = 0;
        let mut ty: i32 = 0;
        move |frm, e| {
            let (x, y) = app::event_coords();

            return match e {
                Event::Push => {
                    tx = x;
                    ty = y;
                    if app::event_mouse_button() == app::MouseButton::Right {
                        menu.popup();
                    }

                    true
                }

                Event::Drag => {
                    let mut tx = tx;
                    let mut ty = ty;
                    let ew = x - tx;
                    let eh = y - ty;

                    if ew < 0 {
                        tx = x;
                    }

                    if eh < 0 {
                        ty = y;
                    }

                    let ew = ew.abs() as u32;
                    let eh = eh.abs() as u32;

                    // offs.begin();
                    // draw::draw_image(&rbga, 0, 0, cg_w as i32, cg_h as i32, ColorDepth::Rgba8).unwrap();
                    // draw::scale(0.5);
                    // // draw::draw_rect_fill(0, 0, 500, 500, Color::from_rgba_tuple((0,0,0,0)));
                    // draw::draw_rect_with_color(tx, ty, ew as i32, eh as i32, Color::Red);
                    // offs.end();
                    // frm.redraw();

                    sel_frm.set_pos(tx, ty);
                    sel_frm.set_size(ew as i32, eh as i32);
                    sel_frm.redraw();
                    frm.redraw();
                    true
                }
                _ => false,
            };
        }
    });

    frm.draw({
        // let offs = offs.clone();
        move |frm| {
            // offs.copy(x, y, sw, sh, 0, 0);
        }
    });
    a.run().unwrap();
}

fn main() {
    let mut bar = create_tray_bar();

    bar.inner_mut().display();
    println!("END");
}
