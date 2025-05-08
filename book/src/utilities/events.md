# Events

There is a simple way to fire and respond to global events.

```rust,noplaypen
pub use hobo::{
    prelude::*,
    create as e,
};

struct MyEvent(u64);

fn make_foo() -> impl hobo::AsElement {
    e::div()
        // etc children and styles
        .component(hobo::events::on(move |&MyEvent(x)| {
            // do something with x
        }))
}

// -- snip --

hobo::events::fire(&MyEvent(123));
```

The subscribers are notified based on event type, so it's better to create new types for different events rather than fire an event with a string or an enum.

Using events, however, is discouraged. The "spooky action at a distance" effect, while often useful, can make the codebase very confusing. Prefer using signals to update the DOM and queries to get entities or components that should be updated.

The implementation in `hobo` isn't unique and there's many crates that implement something similar, but `hobo`'s implementation assumes single-threaded execution.

