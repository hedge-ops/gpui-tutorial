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



## Button Counter

Now that we have the basic scaffolding in place, let's add a counter to our example.

### View

We're going to track the likes of our person, so we we'll add this to our view struct:

```rs
struct Person {
    first_name: SharedString,
    last_name: SharedString,
    likes: u32
}
```

Since the `likes` is a simple unsigned number, and the clone on it is cheap since it's on the stack, we'll use the standard data type here.

### Render

Let's render the `likes` count but also the button that will increment it.

(This is where I am in the process, see the example for a full example)

## Conclusion

This is obviously unfinished, but I'll be on the Zed discord getting feedback and continuing on the journey, updating here. I plan on covering basic components (like buttons, labels), docking, input controls, menuing, app icons, CI, and whatever else I need to get myself up to speed with the framework.

## Outstanding questions for my learning

* If the `SharedString` is basically an `Arc<String>`, how does the framework avoid core contention when ensuring atomicity of the reference counter? [This article](https://blocklisted.github.io/blog/arc_str_vs_string_is_it_really_faster/) makes a good case that `Arc<string>` can be slower in some multithreaded scenarios.
