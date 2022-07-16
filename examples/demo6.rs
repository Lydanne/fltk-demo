use std::iter::Map;

trait Component<D> {
    fn data(&mut self) -> D;

    // fn methods(&mut self) -> Map<&str, dyn Fn() -> ()>;

    fn render(&mut self) {}

    fn h(&mut self, id: &str) {}
}

struct AppData {
    count: i32,
}

struct AppView {
    data: AppData,
}

impl Component<AppData> for AppView {
    fn data(&mut self) -> AppData {
        AppData { count: 0 }
    }

    fn render(&mut self) {
        self.h("Frame")
    }

    fn h(&mut self, id: &str) {}
}

fn main() {}
