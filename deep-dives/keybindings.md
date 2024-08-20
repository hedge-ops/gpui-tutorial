# Keybindings

**Status:** Unfinished

This document is a deep dive in how keybindings work in gpui.

## Definition

The key context is defined in a json file which is an asset:

```json
[
  {
    "context": "menu",
    "bindings": {
      "cmd-l": "application::IncrementLikes",
    }
  },
]
```

## Declaration

A key context is declared on a `div` which states which keyboard shortcuts are available to it. This declaration is a `&str`:

```rs
div()
    .key_context("menu")
```

This takes in a `KeyContext` which is usually a string, that converts to a `KeyContext`.

The conversion process is to `parse` the value, which:

1. Starts with a `default` `KeyContext`, with an empty list of `ContextEntry` instances.
2. Strips any whitespace for the given string
3. Parses the expression, and adds it to `KeyContext` as a string.

But how does `KeyContext` transform from a `&str`?

// MORE TO COME

## Dispatching an Action from a Button

This is done by calling `WindowContext` `dispatch_action`

## Handling an Action

When `on_action` is called on a div, it calls the div's `Interactivity` struct which has an `on_action` method of its own, and adds the action to a list of `action_listeners`, which is a typed list of actions that, during the `DispatchPhase::Bubble` phase, calls the action listeners.
