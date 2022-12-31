// I thought it would be easier to import if it was in 1 file

#[doc = include_str!("../Documentation/ColorsEnum.md")]
pub enum Colors {
  Reset,
  BlackFg,
  RedFg,
  GreenFg,
  YellowFg, 
  BlueFg, 
  MagentaFg,
  CyanFg,
  WhiteFg,
  BrightBlackFg,
  BrightRedFg,
  BrightGreenFg,
  BrightYellowFg,
  BrightBlueFg,
  BrightMagentaFg,
  BrightCyanFg,
  BrightWhiteFg,
  BlackBg,
  RedBg,
  GreenBg,
  YellowBg,
  BlueBg,
  MagentaBg,
  CyanBg,
  WhiteBg,
  BrightBlackBg,
  BrightRedBg,
  BrightGreenBg,
  BrightYellowBg,
  BrightBlueBg,
  BrightMagentaBg,
  BrightCyanBg,
  BrightWhiteBg,
}

#[doc = include_str!("../Documentation/ColorsImpl.md")]
impl Colors {
  pub fn value(&self) -> &str {
    match *self {
        Colors::Reset => "\x1b[0m",
        Colors::BlackFg => "\x1b[30m",
        Colors::RedFg => "\x1b[31m",
        Colors::GreenFg => "\x1b[32m",
        Colors::YellowFg => "\x1b[33m",
        Colors::BlueFg =>  "\x1b[34m",
        Colors::MagentaFg => "\x1b[35m",
        Colors::CyanFg => "\x1b[36m",
        Colors::WhiteFg => "\x1b[37m",
        Colors::BrightBlackFg => "\x1b[90m",
        Colors::BrightRedFg =>  "\x1b[91m",
        Colors::BrightGreenFg => "\x1b[92m",
        Colors::BrightYellowFg => "\x1b[93m",
        Colors::BrightBlueFg => "\x1b[94m",
        Colors::BrightMagentaFg => "\x1b[95m",
        Colors::BrightCyanFg => "\x1b[96m",
        Colors::BrightWhiteFg => "\x1b[97m",
        Colors::BlackBg => "\x1b[40m",
        Colors::RedBg => "\x1b[41m",
        Colors::GreenBg => "\x1b[42m",
        Colors::YellowBg => "\x1b[43m",
        Colors::BlueBg => "\x1b[44m",
        Colors::MagentaBg => "\x1b[45m",
        Colors::CyanBg => "\x1b[46m",
        Colors::WhiteBg => "\x1b[47m",
        Colors::BrightBlackBg => "\x1b[100m",
        Colors::BrightRedBg => "\x1b[101m",
        Colors::BrightGreenBg => "\x1b[102m",
        Colors::BrightYellowBg => "\x1b[103m",
        Colors::BrightBlueBg => "\x1b[104m",
        Colors::BrightMagentaBg => "\x1b[105m",
        Colors::BrightCyanBg => "\x1b[106m",
        Colors::BrightWhiteBg => "\x1b[107m",
    }
  }
}

#[doc = include_str!("../Documentation/ColorsTrait.md")]
pub trait Color {
  fn color(&self, color_enum: Colors) -> String;
}

impl Color for &str {
  fn color(&self, color_enum: Colors) -> String {
    colorize_this(self.to_owned(), color_enum)
  }
}

impl Color for String {
  fn color(&self, color_enum: Colors) -> String {
    colorize_this(self, color_enum)
  }
}

#[doc = include_str!("../Documentation/ColorizeThis.md")]
pub fn colorize_this<S: AsRef<str>>(text: S, color_enum: Colors) -> String {
  format!("{}{}{}", color_enum.value(), text.as_ref(), "\x1b[0m")
}

#[doc = include_str!("../Documentation/ColorizePrintLn.md")]
pub fn colorize_println<S: AsRef<str>>(text: S, color_enum: Colors) {
  println!("{}{}{}", color_enum.value(), text.as_ref(), "\x1b[0m");
}

#[doc = include_str!("../Documentation/ColorizePrint.md")]
pub fn colorize_print<S: AsRef<str>>(text: S, color_enum: Colors) {
  print!("{}{}{}", color_enum.value(), text.as_ref(), "\x1b[0m");
}
#[cfg(test)]
mod tests {
  use crate::*;
  
  #[test]
  fn test_functions() {
    assert_eq!(colorize_this("wow", Colors::BlueFg), "\x1b[34mwow\x1b[0m");
    colorize_print("Foo, Bar? ", Colors::BrightGreenFg);
    colorize_println("I love my parents", Colors::BrightCyanBg);
  }

  #[test]
  fn test_impl() {
    assert_eq!("damn".color(Colors::RedBg), "\x1b[41mdamn\x1b[0m");
    let thing = String::from("Hello");
    assert_eq!(thing.color(Colors::YellowBg), "\x1b[43mHello\x1b[0m");
    println!("{}", "This is so cool".color(Colors::BrightGreenFg));
  }
}
