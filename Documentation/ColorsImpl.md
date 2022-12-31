The implementation that allows the Colors Enum to a &str value

# Examples

```rust
use colorized::Colors;

fn main() {
  assert_eq!(Colors::BlueFg.value(), "\x1b[34m")
}
```
