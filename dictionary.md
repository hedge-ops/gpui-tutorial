# dictionary

A dictionary of terms in gpui.

## Context

Owns all [Models](#model) and [Views](#view).

TODO: there is also a model context

## Div

The main container for elements in the user interface; most views start here and add other elements as children.

## Element

Represents the building blocks of user interface, most of the time starts with a [Div](#div), but there [are other elements too](https://github.com/zed-industries/zed/tree/main/crates/gpui/src/elements).

## SharedString

A string that can be copied easily when the system clones the [Model](#model) after every frame. This is based on an `Arc<String>` which when cloning adds to the reference count of a single `String` on the heap, and thus saves from the processing involved with allocating and pointing to a new string, after every frame.

## State

The underlying state of all [Models](#model) is stored in a `struct`, which has fields with normal types, but for performance reasons uses a [SharedString](#sharedstring).

## Model

Contains [State](#state) and makes up that portion of a [View](#view). Directly used when sharing [State](#state) across an application which [Renders](#render) that state in multiple [Views](#view).


## Render

Using the `Render` trait, transforms a [Model](#model) into an [Element](#element) that can be viewed in the UI.

## View

A [Model](#model) that [Renders](#render) [Elements](#element).

## Update

The `update` method on a [Model](#model), or, by extension, a [View](#view) provides a mutable reference to the [Model](#model) so the [State](#state) can be updated.

Once this updated, [Observers](#observe) are [Notified](#notify).

## Observe

When something needs to react to changes to a [Model](#model), they can call the `observe` method on it, and will be [Notified](#notify) of changes.

## Event

Something that happens in the application that other parts of the application need to respond to.

## Emit

While [Models](#model) can [Update](#update) [State](#state) and [Observers](#observe) can respond to those state updates, this process can also happen in a more straightforward way with [Events](#event).

In this model, the [Model](#model) needs to implement `EventEmitter<T>`, which enables the [Context](#context) to call the `emit` method inside of the [Update](#update), with the [Event](#event) struct instance as an argument.

From this, [Subscribers](#subscribe) receive the event and can respond.

## Subscribe

When something needs to respond to [Events](#event) that a [Model](#model) emit, the [Model Context](#context) provides a `subscribe` method which is called every time the [Event](#event) is [Emittted](#emit). If there are multiple events, they can be matched by type.

## Notify

The [Context](#context) can publish a notification to all [Subscribers](#subscribe) that [State](#state) has changed, giving them an opportunity to update their [Model](#model) or [View](#view).

## App

An application which has its own [context](#context) which owns all [views](#view) and [models](#model). The root object that is instantiated to start the application.

## Window

A container for [Views](#view) that is presented to the user as a separate graphical entity on their screen. It is managed by the operating system and typically includes standard UI elements like a title bar, minimize/maximize buttons, and close button. A window is owned by the [app](#app).

## Event Loop

The continuous process of handling user input, updating application state, and rendering the user interface. The event loop listens for events (such as mouse clicks or keyboard input), processes them, updates the application state accordingly, and then re-renders the UI to reflect these changes. This cycle repeats continuously while the application is running, ensuring that the UI remains responsive to user interactions and system events.
