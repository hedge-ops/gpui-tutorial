use gpui::*;

actions!(application, [IncrementLikes, DecrementLikes]);

struct Person {
    likes: u16,
}

impl Person {
    fn handle_increment(&mut self, _: &IncrementLikes, cx: &mut ViewContext<Self>) {
        println!("incrementing likes");
        self.likes += 1;
        cx.notify();
    }

    fn handle_decrement(&mut self, _: &IncrementLikes, cx: &mut ViewContext<Self>) {
        println!("decrementing likes");
        self.likes -= 1;
        cx.notify();
    }
}

impl Render for Person {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .justify_center()
            .items_center()
            .text_2xl()
            .text_color(rgb(0xffffff))
            .on_action(cx.listener(Self::handle_increment))
            .on_action(cx.listener(Self::handle_decrement))
            .child(format!("Likes: {}", self.likes))
    }
}

fn app_menus() -> Vec<Menu> {
    vec![Menu {
        name: "action_keybindings".into(),
        items: vec![
            MenuItem::action("Increment", IncrementLikes),
            MenuItem::action("Decrement", DecrementLikes),
        ],
    }]
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.activate(true);
            cx.set_menus(app_menus());
            cx.new_view(|_cx| Person { likes: 0 })
        })
        .unwrap();
    });
}
