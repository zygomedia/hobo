# DOM Events and EventHandlerCallback

**Elements** have methods that allow reacting to DOM Events. All of these methods are `snake_case` in the form of `.on_<name>` e.g. `.on_click` or `.on_touch_start`. Not all possible events are supported currently, but adding new ones is very easy - PRs welcome!

```rust,noplaypen
element
    .on_click(move |_| { // the argument here is web_sys::MouseEvent
        element.set_text("I am clicked!");
    })
```

These methods operate by means of, unsurprisingly, adding or modifying a **Component** on the element. The callback itself gets wrapped in [hobo::dom_events::EventHandler](https://docs.rs/hobo/latest/hobo/dom_events/struct.EventHandler.html), which will unsubscribe from DOM when dropped. A **Component** with a `Vec<EventHandler>` is created unless it already exists, then the just created `EventHandler` is just pushed into it.    

It's possible to manage subscribing/unsubscribing manually by calling the functions on raw `web_sys::HtmlElement`s. For example, when you're doing some kind of a slider and you want some logic in `on_mouse_move` even if the mouse leaves the element:

```rust,noplaypen
element
    .on_mouse_down(move |_| {
        // "drag" start
    })
    .component((
        web_sys::window().unwrap().on_mouse_move(move |e| {
            // if dragging, run some dragging logic even once mouse leaves the element
        }),
        web_sys::window().unwrap().on_mouse_up(move |e| {
            // "drag" stop
        }),
    ))
```

Note that the `.on_mouse_move()` and `.on_mouse_up()` aren't actually `web_sys::window()`'s functions - these are extension methods, provided by `hobo` via [hobo::dom_events::RawDomEvents](https://docs.rs/hobo/latest/hobo/dom_events/trait.RawDomEvents.html) extension trait for any [web_sys::EventTarget](https://docs.rs/web-sys/latest/web_sys/struct.EventTarget.html).
