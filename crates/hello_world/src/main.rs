use gpui::*;

struct Person {
    first_name: SharedString,
    last_name: SharedString,
}

impl Render for Person {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {} {}!", &self.first_name, &self.last_name))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Person {
                first_name: "Mick".into(),
                last_name: "Jagger".into(),
            })
        })
        .unwrap();
    });
}
