# hobo-plus
It's a crate for random smaller things that don't fit into `hobo` proper. Currently it exists just as a repository: [https://github.com/zygomedia/hobo-plus](https://github.com/zygomedia/hobo-plus)

* `element_ext::children_diff` - currently the "best effort" of making a "list of things that sometimes change" ergonomic in `hobo`. This construct is easy in VDOM-based frameworks (like React), but challenging otherwise. Ideally, we want to present an interface where the user just provides the data as well as how to convert the data to layout, but not to make the decision about whether to insert/remove/update/reorder existing elements.
* `animation` (and `animation_with_window` for strange use cases) - runs a closure on each animation frame, until the closure returns `false`. Has been useful on quite a few occasions, but is essentially a gnarly pile of boilerplate `web_sys`/`wasm_bindgen` code.
* `svg!` - a macro for defining functions for on-disk SVGs to create them as inline SVGs as `hobo` elements.
* `FileSelect` - turns out it's extremely useful (and common) but also extremely annoying to get a file picker in `hobo` and `web_sys` in general.
* `entity_ext`, `element_ext` and `html_ext` - `hobo`-flavoured extenion traits, that are essentially grab bags of useful methods.
* `socket` module - a simple `postcard`-based socket that buffers messages and automatically reconnects. Likely not useful in all cases, but well suited to how we've been using sockets.
