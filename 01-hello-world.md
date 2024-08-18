# 01 Hello World

In the first section, we'll create a simple hello world program and go over the basics. This is borrowed heavily from the one on the [gpui site](https://www.gpui.rs).

If you're ever having problems following along, the full example exists in the `examples` folder of this repository.

## Introduction

Instead of saying hi to _everyone_, here we're going to say hello to a famous rockstar, Mick Jagger of the Rolling Stones. The chances he will do this tutorial are near zero, but one can always hope!

## View

To begin we need to define how we want to present the greeting to Mick Jagger. This is done by defining a [view](dictionary.md#view). A _view_ is the simplest [element](dictionary.md#element) we can use to [render](dictionary.md#render) UI in gpui. It is made up of two parts: (1) data that we will show (i.e., the [state](dictionary.md#state)), (2) the way we present (or [render](dictionary.md#render)) that data to a UI.

### State

The first part, the data is represented in a simple struct as the [state](dictionary.md#state):

```rs
struct Person {
    first_name: SharedString,
    last_name: SharedString,
}
```

Note that this uses [`SharedString`](dictionary.md#sharedstring), a type provided by the gpui framework, because the structure will be cloned as every frame is shown to the user. With this level of cloning, we want to use `SharedString` so we copy a reference to the string (through an underlying `Arc`), rather than the entire string itself. While the simple `String` here would work, it would be slower. The slow performance with `String`s would be more pronounced with larger strings.

For other elements on the stack, like integers, characters, or floats, we don't need any special alternative types since their `clone` is cheap.

### Rendering

Now that the data, or [state](dictionary.md#state) we want to show is defined, it's time to define _how_ that state is shown on the screen. We do this by implementing the [Render](dictionary.md#render) trait which makes the struct a [View](dictionary.md#view).

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

The render method translates the state of the view into gpui elements, using a tailwind-inspired API. The chained `bg`, `size_full`, `justify_center` methods should be intuitive to someone who has experience with [tailwind](https://tailwindcss.com).

The [`div`](dictionary.md#div) here is conceptually similar to the div in html, but in gpui, `div` is the root container [element](dictionary.md#element) for almost everything you will make, including buttons, input fields, and panels. There is not a 1:1 mapping of what is in gpui and what you find in html; you will build the building blocks yourself.

## App

We have defined a person [view](dictionary.md#view) and it's time to initialize an application and window to show it.

We do this inside of the `main` function:

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

This renders the `Person` in the single [window](dictionary.md#window) of the [application](dictionary.md#app), with an event loop (kicked off by `run`) that waits for more input from the user.

## Building

When you're ready to run, run `cargo run` and you get this:

![Hello World Window](/assets/01-hello-world.png)

If you're having issues (as I did), refer to the setup documentation [above](#installation) which provides excellent troubleshooting instructions.

## Challenge

Now that we have a scaffolding, play with the example. Can you change it to your name? Can you change the background colors?

## Conclusion

In this section we learned how to define a [view](dictionary.md#view), by first defining its [state](dictionary.md#state) and how to [render](dictionary.md#render) that state.

We then created an [app](dictionary.md#app) that shows a [window](dictionary.md#window) of our view.

Mick Jagger is now ready to learn gpui, with an app that will greet him!

Next, we'll get into some interactivity with [02 likes counter](02-likes-counter.md).
