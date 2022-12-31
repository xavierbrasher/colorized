# Colorized

Colorized is a simple rust library that allows for you to display color to the console

# Getting Started

Just add the line on [crates.io](https://crates.io/crates/colorized) to your Cargo.toml to install it

# Usage

To use Colorized all you have to do is

```rust
use colorized::*;

fn main() {
  println!("{}", "This is so cool".color(Colors::BrightGreenFg));
  let this: String = colorize_this("wowzers", Colors::BrightBlackBg);
  colorize_print("Wow this is great", Colors::BrightCyanFg);
  colorize_println("Wow this is great", Colors::BrightCyanFg);
}
```

# Documentation

Colorized uses ASCI Codes and thats how it remains very simple. In the source code you may be able to tell that it is my first time making and uploading a libary but I hope this libary could be useful.

# Colors Enum

This enum makes it easy to access all of the colors and color values.

## Examples

```rust
use colorized::*;

fn main() {
  println!("This is with color:  {} woah {}", Colors::BlueFg.value(), Colors::Reset.value());
  println!("Same thing but easier: {}", "Oh my".color(Colors::BrightCyanBg));
  let coloredString = colorize_this("This is color", Colors::YellowFg);
}
```

## Implementation

The implementation that allows the Colors Enum to a &str value

### Examples

```rust
use colorized::Colors;

fn main() {
  assert_eq!(Colors::BlueFg.value(), "\x1b[34m")
}
```

# Colors Traits

This allows Strings and &str to have color with .color(COLOR_ENUM)

## Examples

```rust
use colorized::*;

fn main() {
    println!("{}", "Wow".color(Colors::CyanFg));
    let john: String = String::from(":)");
    println!("{}",  john.color(Colors::BrightMagentaBg));
}
```

# Colorize Print

This is a funcition that prints a full colored sentence without a new line charactor

## Examples

```rust
use colorized::*;

fn main() {
  colorize_print("Wow this is great", Colors::BrightCyanFg);
}
```

# Colorize PrintLn

This is a funcition that prints a full colored sentence with a new line charactor

## Examples

```rust
use colorized::*;

fn main() {
  colorize_println("Wow this is great", Colors::BrightCyanFg);
}
```

# Colorize this

This function adds color to anything that isn't implemented

## Examples

```rust
use colorized::*;

fn main() {
  let this: String = colorize_this("wowzers", Colors::BrightBlackBg);
}
```

# Format of color

Colors::{COLORNAME}\_<FG or BG>

All Posible Colors

1. black
1. red
1. green
1. yellow
1. blue
1. magenta
1. cyan
1. white
1. bright_black
1. bright_red
1. bright_green
1. bright_yellow
1. bright_blue
1. bright_magenta
1. bright_cyan
1. bright_white

# License

MIT
