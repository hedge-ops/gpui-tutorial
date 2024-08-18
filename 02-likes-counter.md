# Likes Counter

If you haven't already completed the first tutorial, [start there](01-hello-world.md).

## Introduction

We've given up on getting Mick Jagger to do this tutorial, but we do want to create an app for his fans to count up how many people like him. This will introduce us to [state](dictionary.md) mutation and interactivity.

## View

### State

We want to start tracking how many likes our `Person` [view](dictionary.md#view) has, so we add that to the [state](dictionary.md#state):

```rust
struct Person {
    first_name: SharedString,
    last_name: SharedString,
    likes: u16,
}
```

### Render

We're now rendering three things: (1) the name person we're showing, (2) how many likes that person has, and (3) an ability to like that person, via a button.

#### Parent Render

We'll start with the parent render, which will change to be a container [div](dictionary.md#div) [element](dictionary.md#element), and call other functions to render the different parts (below). In future tutorials, we'll break this out into different [elements](dictionary.md#element), but for this tutorial section we'll keep it as simple as possible.

Here is our parent render:

```rust
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
            .child(self.render_name())
            .child(self.render_likes())
            .child(self.render_like_button(cx))
    }
}
```

Here we have a container that lays out children in a column (`flex-col()`) with a small gap betwen elements (`gap-2()`).

#### Name Render

The greeting is similar to what it was in the [previous tutorial](01-hello-world.md):

```rust
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
}
```

This in effect is showing the name, but with a bit larger text than before (`text_2xl()`). The `&self` gives us read access to the [state](dictionary.md#state) we defined above.

### Likes Render

On the next line, we want to render the `likes` that we just defined to be a part of our [state](dictionary.md#state):

```rust
impl Person {
    // render_greeting goes here

    fn render_likes(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("Likes: {}", self.likes))
    }
```

This is straightforward as well; the likes is accessible through `self` (and the [state](dictionary.md#state) defined above. The font is a little smaller (`text_xl()`) but this just demonstrates that you can compose UI any way you want, similar to how you would in a website.

### Like Button Render

#### Reasoning for the Missing Built-in Button

In every other UI framework I've used the framework provided an out-of-the-box button widget that allowed you to create a button. While zed is open source, and in their [ui](https://github.com/zed-industries/zed/tree/main/crates/ui) crate their is a [button](https://github.com/zed-industries/zed/tree/main/crates/ui/src/components/button), that crate is under GPL license, meaning you can only use it for code you also license as GPL and open source.

This was confusing at first. Why do the people behind zed want me to use gpui, but not this? After some digging in their Discord Server, they said that they want to encourage people to create their own experiences, and therefore their own component library is really meant for the zed experience.

I view this as one of the few deficiencies I was able to find in the framework, because yeah I like to customize, but help me onboard by giving me something better to start with. As I onboard, I might create a component library and open source it with a more permissive license so people can onboard to the framework more quickly.

#### Rendering the Like Button

So with that out of the way, the like button is actually going to be a [div](dictionary.md#div) [element](dictionary.md#element) that is responsibe to when the mouse hovers over it, and when the left mouse button is clicked. Here is the code:

```rust
impl Person {
    // other render methods omitted

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
```

Most of this is the layout code you are already familiar with, i.e. `flex()`, `text_xl()` to define the text size, `border_2()` to put a border around the button, `p_2()` to add some padding to the button, `rounded_lg()` to make the border rounded and make it look more like a traditional button, `border_color()`, `text_color()`, and `bg()` to define the colors. Nothing really special here if you come from a tailwind background or are getting used to the way we lay out UI in gpui.

The two new parts here that require some explanation are the style changes on hover, and the action to take when clicking the mouse button.

When hovering, we change the style with this line:

```rust
.hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
```

The `hover()` method takes a closure (an anonymous function) as an argument, which receives a `style` parameter and returns a modified style. In this case, it's changing the background color of the element to `BUTTON_HOVER_COLOR` when hovered. This creates a visual feedback for the user, indicating that the element is interactive.

Now we can hook up our event handler when the mouse left button is clicked:

```rust
.on_mouse_down(MouseButton::Left, cx.listener(Self::handle_increment))
```

The `on_mouse_down` method takes an argument of which mouse button we are tracking, and a listener as the second argument, provided by our [context](dictionary.md#context). The `cx.listener` method takes a function argument `Self::handle_increment` which we define as a defined function:

```rust
fn handle_increment(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
    self.likes += 1;
    cx.notify();
}
```

In this method, we update the `likes` field of the mutable `Person` [view](dictionary.md#view). Then we notify the [context](dictionary.md#context) that [state](dictoinary.md#state) has changed, so it knows to redraw the UI.

## Conclusion

When you run `cargo run` you'll see:

![Likes Counter Window](/assets/02-likes-counter.png)


There you have it, we were quickly able to pivot our application into something that can track people who like Mick Jagger. In doing so we learned how to create a responsive button that handles a click event.
