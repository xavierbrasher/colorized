This allows Strings and &str to have color with .color(COLOR_ENUM)

# Examples

```rust
use colorized::*;

fn main() {
    println!("{}", "Wow".color(Colors::CyanFg));
    let john: String = String::from(":)");
    println!("{}",  john.color(Colors::BrightMagentaBg));
}
```
