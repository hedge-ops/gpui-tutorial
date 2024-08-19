# gpui Tutorial

## Summary

[GPUI](https://www.gpui.rs) is a graphics processor accelerated user interface frameowork, created by the makers of [Zed](https://zed.dev) to make the fastest user interface possible, written in Rust. It's currently an [internal project](https://github.com/zed-industries/zed/tree/main/crates/gpui) in the zed project, but will soon be its own crate. It's licensed under the [Apache license](https://github.com/zed-industries/zed/blob/main/crates/gpui/LICENSE-APACHE), which makes it usable by commercial projects.

Currently there is not a lot of documentation, so this tutorial aims to take you through the basics of the framework to get started on your own project.

## Sections

* [00 Prerequisites](00-prerequisites.md), to make sure you know Rust and have your machine set up properly.
* [01 Hello World](01-hello-world.md), where we learn the basics of creating a gpui app.
* [02 Likes Counter](02-likes-counter.md), where we update state when clicking a button.

To understand the basic terms, see [the dictionary](dictionary.md). To read more on how I learned the framework, see [resources](resources.md).

## Future Topics

* Elements -> make button its own element and compose the three elements, add a click handler and disabled property
* Conditional state -> make it so the button is disabled when reaching 10 likes.
* Actions and keybindings -> add cmd+l as an action for like, and map the click event to that same action.
* Menus -> define menus and map those to actions as well. Menu is disabled too when the action is not available.
* App Icon -> add the application icon as an asset so we have a real app.
* Shared state with Models -> add a dislike button (with cmd+d as keybinding), which is disabled when likes is 0.
* Icons -> turn like and dislike into icon buttons (introducing the svg element)
* Input Controls - give the user the opportunity to update the name
* Avatar -> using the `img` element

### Missing

* `RenderOnce` vs `Render` - probably a part of the Elemnts tutorial
* lists (and uniform list)
* notify/observe with `update` from context
* emit/event
* async app context
* testing
* Window fit & feel -> see positioning, shadow examples
* Animation -> TBD
* CI -> build the thing
* Anchored elements


## Outstanding Questions

* Is render once for sub-elements that are immutable? Asking...
* If the `SharedString` is basically an `Arc<String>`, how does the framework avoid core contention when ensuring atomicity of the reference counter? [This article](https://blocklisted.github.io/blog/arc_str_vs_string_is_it_really_faster/) makes a good case that `Arc<string>` can be slower in some multithreaded scenarios.
