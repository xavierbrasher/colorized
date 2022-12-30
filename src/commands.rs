pub fn colorize_this<S: Into<String>>(text: S, color_code: String) -> String {
  format!("{}{}{}", color_code, text.into(), "\x1b[0m")
} 

pub fn colorize_println<S: Into<String>>(text: S, color_code: String) {
  println!("{}{}{}", color_code, text.into(), "\x1b[0m")
}

pub fn colorize_print<S: Into<String>>(text: S, color_code: String) {
  print!("{}{}{}", color_code, text.into(), "\x1b[0m")
}