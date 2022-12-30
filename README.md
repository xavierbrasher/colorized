# Colorized

Colorized is a simple rust library that allows for you to display color to the console

# Getting Started

Just add the line on [crates.io](https://crates.io/crates/colorized) to your Cargo.toml to install it

# Usage

To use Colorized all you have to do is

```rust
use colorized::{color_definations::*, commands::*};

fn main() {
  let fg: ColorsFg = ColorsFg::new();
  let bg: ColorsBg = ColorsBg::new();

  println!("{}", colorize_this("Wow this is amazing", bg.green_bg));
  colorize_println("This is just so efficent", fg.bright_red_fg);
}
```

# Documentation

Colorized uses ASCI Codes and thats how it remains very simple. In the source code you may be able to tell that it is my first time making and uploading a libary but I hope this libary could be useful.

## Example of how you could use this

```rust
use colorized::color_definations::*;

fn main() {
  let bg = ColorsBg::new();
  println!("{} Foo, Bar?? {}", bg.bright_red_bg, bg.reset_bg);
}
```

This shows that this crate is just a package of easy to access ascii codes with a few helper functions.

## Helper Function Examples

```rust
use colorized::{color_definations::*, commands::*};

fn main() {
  let bg = ColorsBg::new();
  let fg = ColorsFg::new();

  println!("{}", colorize_this("This returns a colorized version of this", fg.bright_green_fg));
  colorize_println("this does the same thing but with a new line char", fg.magenta_fg);
  colorize_print("This just prints it without a new line char ", bg.cyan_bg);
}
```

## Format of color

{ColorName}\_{FG or BG}

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
