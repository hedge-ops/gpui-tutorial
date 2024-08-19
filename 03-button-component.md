# 03 Button Component

If you haven't already done the [likes counter](02-likes-counter.md), start there.

## Introduction

In the previous lesson, we created a likes counter application with elements, but one of those elements was a button, and we'd like to reuse that button throughout our application. Let's do a bit of refactoring and set that up.

## Macro Dependencies

To easily create components, we're going to take advantage of the `gpui_macros` crate, which sits alongside the `gpui` crate and is also licensed under Apache.

Let's add this to our `Cargo.toml`:

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_macros = { git = "https://github.com/zed-industries/zed" }
```

## Button struct

With that in place, we'll define the [state](dictionary.md#state) of our button:

```rust
#[derive(IntoElement)]
struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}
```

This struct has a few new concepts we haven't yet encountered:

1. `#[derive(IntoElement)]` - remember that all `render` functions return `impl IntoElement`, and all `child` calls take an `impl IntoElement` parameter. So we need a way for our button to conform to that, and the `gpui_macro` crate gives us this nice shorcut way of doing so.
2. `id: ElementId` - since our button is tracking events across frames, we need to identify it so the system can turn a `MouseUp` event and `MouseDown` event into a `ClickEvent`. This `id` field gives us the ability to identify the element.
3. `on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>` - wow this is a mouthful! In short the `Option<T>` makes the `on_click` optional, and its value is a pointer (`Box`) to a function (`Fn`) that takes `ClickEvent` and `WindowContext` arguments. And finally the `+ 'static` part says that this handler can be used for the duration of the application, and won't bring in anything non-static to its scope. We can guarantee this because the `App` (and therefore `AppContext` owns all of these elements, and keeps them alive for the duration of the program).

Note that since `#[derive(IntoElement)]` expects for `RenderOnce` to be implemented, the solution does not yet compile, but be assured that we are quickly getting there.

## Button Convenience methods

To make this look and feel like a real component, we want to create a `new` method for `Button`, and a convenience `on_click` method for when users want to define a function (remember, it's optional). We do this next:

```rust
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
```

Note that the `id.into()` allows us to pass in `&str` to the constructor, which makes the whole experience more delightful.

These methods are technically not necessary but help make the component _feel_ like a gpui component.

## Button Rendering

It's time to implement `RenderOnce` for our `Button`:

```rust
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
```

Let's go over some new concepts:

1. `RenderOnce` this means that we are not going to change any state related to this button, so gpui will call this every time it rerenders the parent window. Not having to keep track of this UI makes the framework have less memory and is easier to reason about. So the rule is: if you have a component that does not change, use `RenderOnce`. With this in place, our `#[derive(IntoElement)]` will now work, because it wraps the `RenderOnce` into something that will turn into an element the framework can use.
2. `id(self.id)` this assigns our button a unique id so the framework can track events for it, turning a mouse down, followed by a mouse up into a click event. Without this definition, we would not be able to define the `on_click` below.
3. `when_some(self.on_click, |this, on_click|)` this is syntatic sugar that basically says, if on_click has a value, call this function. Remember, we made `on_click` optional.
4. `this.on_click(move |evt, cx| (on_click)(evt, cx))` this is inside the `when_some` block above, this defines an `on_click` for the parent `Div` element (remember this is what we called `when_some` on), and when that is fired, calls the `Button`'s `on_click` function.

This is all a lot and definitely caused me to stare at it quite a bit to understand what was going on. But at the end, the implementation was simple: we have an optional `on_click` handler to a button with an `id` that is passed the click event of its child `Div` element. And since the `Button` implements the `RenderOnce` trait, it will be called on every frame.

## Using our Button

Now in our parent `Person` [view](dictionary.md#view) we can use our `Button` component as a child, instead of calling the `render_likes_button` method as we did before. Here is the relavant change:

```rust
.child(
    Button::new("like-button", "Like".into())
        .on_click(cx.listener(Self::handle_increment)),
)
```

Here a `Button` with an id of `like-button` is created, with the text of `Like`. We define an `on_click` method, to call `handle_increment`. Since this is a click event instead of a mouse up event, we slightly change the signature of the `handle_increment` method:

```rust
fn handle_increment(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
    self.likes += 1;
    cx.notify();
}
```

## Conclusion

And there you have it! We now have created a reusable component. In the course of this, we learned how to define event handlers, render immutable components with `RenderOnce`, how some elements can have and `id` (and why you would want one), and how to easily transform our component into an `Element` with the `#[derive(IntoElement)]` macro.

Since this is just a refactoring, we won't include the screenshot of the app this time.

Next we'll look at how to drive our application with the keyboard with [actions](dictionary.md#action).
