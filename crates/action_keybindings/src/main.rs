use gpui::*;
use prelude::FluentBuilder;

const BACKGROUND_COLOR: u32 = 0x1E2027;
const FOREGROUND_COLOR: u32 = 0xE6E6E6;
const BORDER_COLOR: u32 = 0x2D3039;
const BUTTON_BACKGROUND_COLOR: u32 = 0x3B82F6;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x60A5FA;

actions!(application, [IncrementLikes]);

#[derive(IntoElement)]
struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
        }
    }

    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, cx| (on_click)(evt, cx))
            })
            .child(self.label)
    }
}

struct Person {
    first_name: SharedString,
    last_name: SharedString,
    likes: u16,
}

impl Person {
    fn render_name(&self) -> impl IntoElement {
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

    // fn handle_increment(&mut self, _: &IncrementLikes, cx: &mut ViewContext<Self>) {
    //     println!("handling action");
    //     self.likes += 1;
    //     cx.notify();
    // }
}

impl Render for Person {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("person-view")
            .on_action(move |&IncrementLikes, _cx| {
                eprintln!("hello");
            })
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .on_click(|_, cx| {
                println!("handling click at the div level");
                cx.dispatch_action(Box::new(IncrementLikes))
            })
            .child(self.render_name())
            .child(self.render_likes())
            .child(
                Button::new("like-button", "Like".into()).on_click(cx.listener(|_, _, cx| {
                    println!("handling click, dispatching action IncrementLikes");
                    cx.dispatch_action(Box::new(IncrementLikes));
                })),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            #[cfg(target_os = "macos")]
            let binding = KeyBinding::new("cmd-L", IncrementLikes, None);

            #[cfg(not(target_os = "macos"))]
            let binding = KeyBinding::new("ctrl-L", IncrementLikes, None);

            println!("Binding: {:?}", binding);
            cx.bind_keys(vec![binding]);

            cx.new_view(|_cx| Person {
                first_name: "Mick".into(),
                last_name: "Jagger".into(),
                likes: 0,
            })
        })
        .unwrap();
    });
}
