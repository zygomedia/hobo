# Colors

Color property macros like `css::color` and `css::fill` and the like have shorthands for full-alpha RGB colors as well as grayscale.

```rust,noplaypen
css::color::rgb(0xFF_00_00), // same as css::color::rgba(0xFF_00_00_FF) or #F00 in css
css::color::gray(0xAD), // same as css::color::rgba(0xAD_AD_AD_FF) or #ADADAD in css
```

CSS named colors also can be used
```rust,noplaypen
css::color::rgba(css::colors::PALEVIOLETRED),
css::color::rgba(css::colors::GREEN),
```
