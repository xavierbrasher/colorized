# Colorized

Colorized is a simple rust library that allows for you to display color to the console

# Getting Started

Just add "colorized = 0.8.5" to your Cargo.toml to install it

# Usage

To use Colorized all you have to do is

```
use Colorized::{color_definations::*, commands::*}

fn main() {
  let fg: ColorsFg = ColorsFg::new();
  let bg: ColorsBg = ColorsBg::new();

  println!("{}", colorize_this("Wow this is amazing", bg.green_bg));
  colorize_println("This is just so efficent", fg.bright_red_fg)
}
```

# License

MIT
