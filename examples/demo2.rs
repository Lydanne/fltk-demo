use flemish::{
    button::Button, color_themes, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings,
};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::BLACK_THEME),
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - fltk-rs")
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::IncrementPressed => {
                self.value += 1;
            }
            CounterMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        Button::default()
            .with_label("Increment")
            .on_event(CounterMessage::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        Button::default()
            .with_label("Decrement")
            .on_event(CounterMessage::DecrementPressed);
        col.end();
    }
}
