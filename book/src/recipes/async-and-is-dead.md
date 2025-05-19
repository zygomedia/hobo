# Async and .is_dead()

Be careful carelessly accessing entities from an async context. Make sure to check that your entity is still mounted by the time your async computations finish and you're trying to change something.

```rust,noplaypen
use std::future::Future;

pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
    wasm_bindgen_futures::spawn_local(async move { if let Err(e) = x.await {
        log::error!("{:?}", e);
    }});
}

e::div()
    .tap(move |&element| spawn_complain(async move {
        let value = do_some_request_or_something().await?;
        if element.is_dead() { return Ok(()); }
        element.set_text(value);
        Ok(())
    }))
```

This isn't necessary outside of async context because wasm is single-threaded so your element can't get unmounted due to user actions, but in some complex scenarios it might be useful anyway. Frequent culprits for issues with this are `.child_signal()` (which generates a new entity on every signal update) and `.replace()` (which replaces with a new entity).

You can also use [.spawn()](https://docs.rs/hobo/latest/hobo/entity/trait.AsEntity.html#method.spawn) and [.spawn_in()](https://docs.rs/hobo/latest/hobo/entity/trait.AsEntity.html#method.spawn_in) to spawn a future that will be cancelled when the entity is removed. This is fine to do since if our `div` is removed - `.set_text()` will not be called.

```rust,noplaypen
e::div()
    .spawn_in(async move |&element| {
        let value = do_some_request_or_something().await?;
        element.set_text(value);
    })
```
