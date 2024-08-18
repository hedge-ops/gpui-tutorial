use gpui::*;

const BACKGROUND_COLOR: u32 = 0x1E2027;
const FOREGROUND_COLOR: u32 = 0xE6E6E6;
const BORDER_COLOR: u32 = 0x2D3039;
const BUTTON_BACKGROUND_COLOR: u32 = 0x3B82F6;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x60A5FA;

struct Person {
    first_name: SharedString,
    last_name: SharedString,
    likes: u16,
}

impl Person {
    fn render_greeting(&self) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(BACKGROUND_COLOR))
            .justify_center()
            .items_center()
            .text_2xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("{} {}", &self.first_name, &self.last_name))
    }

    fn render_likes(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("Likes: {}", self.likes))
    }

    fn handle_increment(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        self.likes += 1;
        cx.notify();
    }

    fn render_like_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_increment))
            .child("Like")
    }
}

impl Render for Person {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child(self.render_greeting())
            .child(self.render_likes())
            .child(self.render_like_button(cx))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Person {
                first_name: "Mick".into(),
                last_name: "Jagger".into(),
                likes: 0,
            })
        })
        .unwrap();
    });
}
