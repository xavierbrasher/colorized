This enum makes it easy to access all of the colors and color values.

# Examples

```rust
use colorized::*;

fn main() {
  println!("This is with color:  {} woah {}", Colors::BlueFg.value(), Colors::Reset.value());
  println!("Same thing but easier: {}", "Oh my".color(Colors::BrightCyanBg));
  let coloredString = colorize_this("This is color", Colors::YellowFg);
}
```

# Implementation

The implementation that allows the Colors Enum to a &str value

## Examples

```rust
use colorized::Colors;

fn main() {
  assert_eq!(Colors::BlueFg.value(), "\x1b[34m")
}
```
