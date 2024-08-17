# gpui

## Summary

[GPUI](https://www.gpui.rs) is a graphics processor accelerated user interface frameowork, created by the makers of [Zed](https://zed.dev) to make the fastest user interface possible, written in Rust. It's currently an [internal project](https://github.com/zed-industries/zed/tree/main/crates/gpui) in the zed project, but will soon be its own crate. It's licensed under the [Apache license](https://github.com/zed-industries/zed/blob/main/crates/gpui/LICENSE-APACHE), which makes it usable by commercial projects.

## Installation

To use gpui, add a reference to it in the `Cargo.toml` file:

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
```

Since gpui is a part of Zed's codebase, we can borrow their setup instructions to get set up to do gpui development. Follow the instructions in the `Dependencies` section for [macOS](https://github.com/zed-industries/zed/blob/main/docs/src/development/macos.md#dependencies), [Linux](https://github.com/zed-industries/zed/blob/main/docs/src/development/linux.md#dependencies), or [Windows](https://github.com/zed-industries/zed/blob/main/docs/src/development/windows.md#dependencies). You'll ignore the other sections, but there might be troubleshooting at the bottom that will be helpful.

## 01 Scaffolding

### View

To scaffold a simple project, first define a `View` struct with data:

```rs
struct PersonView {
    first_name: SharedString,
    last_name: SharedString,
}
```

View structs should use `SharedString` for strings, but can use primitives like i32. The memory model of gpui enables sharing, so the data types used in these views, need to play along with that for data types on the heap.

This struct needs to implement the `Render` trait:

```rs
impl Render for PersonView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
```

The render method basically translates the state of the view into gpui elements, using a tailwind-inspired API. The `div` here is similar to that in html, but in gpui, `div` is the root container element for everything you will make, including buttons, input fields, and panes. There is not a 1:1 mapping of what is in gpui and what you find in html; you have to build the building blocks yourself.

### App

Now that we have a view that can render, we are able to initialize our app with it, inside of the main function:

```rs
fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| PersonView {
                first_name: "Mick".into(),
                last_name: "Jagger".into(),
            })
        })
        .unwrap();
    });
}
```

This renders the `PersonView` in the single window of the application, with an event loop (kicked off by `run`) that waits for more input from the user.

### Building

When you're ready to run, run `cargo run` and you get this:

![Scaffolding Window](/assets/scaffolding.png)

This is the basic scaffolding for setting up a gpui application.
