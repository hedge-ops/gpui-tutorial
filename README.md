# gpui Tutorial

## Summary

[GPUI](https://www.gpui.rs) is a graphics processor accelerated user interface frameowork, created by the makers of [Zed](https://zed.dev) to make the fastest user interface possible, written in Rust. It's currently an [internal project](https://github.com/zed-industries/zed/tree/main/crates/gpui) in the zed project, but will soon be its own crate. It's licensed under the [Apache license](https://github.com/zed-industries/zed/blob/main/crates/gpui/LICENSE-APACHE), which makes it usable by commercial projects.

Currently there is not a lot of documentation, so this tutorial aims to take you through the basics of the framework to get started on your own project.

## Installation

To use gpui, add a reference to it in the `Cargo.toml` file:

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
```

Since gpui is a part of Zed's codebase, we can borrow their setup instructions to get set up to do gpui development. Follow the instructions in the `Dependencies` section for [macOS](https://github.com/zed-industries/zed/blob/main/docs/src/development/macos.md#dependencies), [Linux](https://github.com/zed-industries/zed/blob/main/docs/src/development/linux.md#dependencies), or [Windows](https://github.com/zed-industries/zed/blob/main/docs/src/development/windows.md#dependencies). You'll ignore the other sections, but there might be troubleshooting at the bottom that will be helpful.

## 01 Scaffolding

In this section, we'll create a basic project that shows a window, and get the absolute basics out of the way upon which we can build.

### View

A view is the simplest element we can use to render UI in gpui. It has two elements: (1) data that we will render, (2) the way we render that data to a UI.

#### Data

The first element, the data is represented in a simple struct:

```rs
struct Person {
    first_name: SharedString,
    last_name: SharedString,
}
```

This structure uses `SharedString`, a type provided by the framework, because the structure will be cloned every frame. With this level of cloning, we want to use `SharedString` so we copy a reference to the string (through an underlying `Arc`), rather than the entire string itself. While the simple `String` here would work, it would be less performant, especially for larger strings.

For other elements on the stack, like integers, characters, or floats, we can simply use those types since their clone is cheap.

#### Rendering

Now that the struct has data defined, to make it a `View`, implement the `Render` trait:

```rs
impl Render for Person {
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

The render method translates the state of the view into gpui elements, using a tailwind-inspired API. The `bg`, `size_full`, `justify_center` methods should be intuitive to someone who has experience with [tailwind](https://tailwindcss.com).

The `div` here is conceptually similar to the div in html, but in gpui, `div` is the root container element for everything you will make, including buttons, input fields, and panels. There is not a 1:1 mapping of what is in gpui and what you find in html; you will build the building blocks yourself.

### App

Now that we have a view that can render, we are able to initialize our app with it, inside of the `main` function:

```rs
fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Person {
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

If you're having issues (as I did), refer to the setup documentation [above](#Installation) which provides excellent troubleshooting instructions.

### Challenge

Now that we have a scaffolding, play with the example. Can you change it to your name? Can you change the background colors?

## Conclusion

This is obviously unfinished, but I'll be on the Zed discord getting feedback and continuing on the journey, updating here. I plan on covering basic components (like buttons, labels), docking, input controls, menuing, app icons, CI, and whatever else I need to get myself up to speed with the framework.
