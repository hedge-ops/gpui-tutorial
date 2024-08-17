use gpui::*;

struct Theme {
    background: Rgba,
    foreground: Rgba,
    border: Rgba,
    button_background: Rgba,
    button_foreground: Rgba,
    button_hover: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: rgb(0x1E2027),
            foreground: rgb(0xE6E6E6),
            border: rgb(0x2D3039),
            button_background: rgb(0x3B82F6),
            button_foreground: rgb(0xFFFFFF), // Pure white
            button_hover: rgb(0x60A5FA),
        }
    }
}

struct Person {
    theme: Theme,
    first_name: SharedString,
    last_name: SharedString,
    likes: u16,
}

impl Person {
    fn new(first_name: SharedString, last_name: SharedString) -> Self {
        Person {
            theme: Theme::default(),
            first_name,
            last_name,
            likes: 0,
        }
    }

    fn render_greeting(&self) -> impl IntoElement {
        div()
            .flex()
            .bg(self.theme.background)
            .justify_center()
            .items_center()
            .text_2xl()
            .text_color(self.theme.foreground)
            .child(format!("{} {}", &self.first_name, &self.last_name))
    }

    fn render_likes(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(self.theme.foreground)
            .child(format!("Likes: {}", self.likes))
    }

    fn handle_increment(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        self.likes += 1;
        cx.notify();
    }

    fn render_increment_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(self.theme.border)
            .text_color(self.theme.button_foreground)
            .bg(self.theme.button_background)
            .hover(|style| style.bg(self.theme.button_hover))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_increment))
            .child("Like")
    }
}

impl Render for Person {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(self.theme.background)
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child(self.render_greeting())
            .child(self.render_likes())
            .child(self.render_increment_button(cx))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Person::new("Mick".into(), "Jagger".into()))
        })
        .unwrap();
    });
}
