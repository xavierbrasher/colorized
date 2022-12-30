pub struct ColorsFg { 
  pub reset_fg: String,
  pub black_fg: String,
  pub red_fg: String,
  pub green_fg: String,
  pub yellow_fg: String,
  pub blue_fg: String,
  pub magenta_fg: String,
  pub cyan_fg: String,
  pub white_fg: String,
  pub bright_black_fg: String,
  pub bright_red_fg: String,
  pub bright_green_fg: String,
  pub bright_yellow_fg: String,
  pub bright_blue_fg: String,
  pub bright_magenta_fg: String,
  pub bright_cyan_fg: String,
  pub bright_white_fg: String,
}

impl ColorsFg {
  pub fn new() -> ColorsFg {
    ColorsFg {
      reset_fg: String::from("\x1b[0m"), 
      black_fg: String::from("\x1b[30m"),
      red_fg: String::from("\x1b[31m"),
      green_fg: String::from("\x1b[32m"),
      yellow_fg: String::from("\x1b[33m"),
      blue_fg: String::from("\x1b[34m"),
      magenta_fg: String::from("\x1b[35m"),
      cyan_fg: String::from("\x1b[36m"),
      white_fg: String::from("\x1b[37m"),
      bright_black_fg: String::from("\x1b[90m"),
      bright_red_fg: String::from("\x1b[91m"),
      bright_green_fg: String::from("\x1b[92m"),
      bright_yellow_fg: String::from("\x1b[93m"),
      bright_blue_fg: String::from("\x1b[94m"),
      bright_magenta_fg: String::from("\x1b[95m"),
      bright_cyan_fg: String::from("\x1b[96m"),
      bright_white_fg: String::from("\x1b[97m"),
    }
  }
}

pub struct ColorsBg { 
  pub reset_bg: String,
  pub black_bg: String,
  pub red_bg: String,
  pub green_bg: String,
  pub yellow_bg: String,
  pub blue_bg: String,
  pub magenta_bg: String,
  pub cyan_bg: String,
  pub white_bg: String,
  pub bright_black_bg: String,
  pub bright_red_bg: String,
  pub bright_green_bg: String,
  pub bright_yellow_bg: String,
  pub bright_blue_bg: String,
  pub bright_magenta_bg: String,
  pub bright_cyan_bg: String,
  pub bright_white_bg: String,
}

impl ColorsBg {
  pub fn new() -> ColorsBg {
    ColorsBg {
      reset_bg: String::from("\x1b[0m"), 
      black_bg: String::from("\x1b[40m"),
      red_bg: String::from("\x1b[41m"),
      green_bg: String::from("\x1b[42m"),
      yellow_bg: String::from("\x1b[43m"),
      blue_bg: String::from("\x1b[44m"),
      magenta_bg: String::from("\x1b[45m"),
      cyan_bg: String::from("\x1b[46m"),
      white_bg: String::from("\x1b[47m"),
      bright_black_bg: String::from("\x1b[100m"),
      bright_red_bg: String::from("\x1b[101m"),
      bright_green_bg: String::from("\x1b[102m"),
      bright_yellow_bg: String::from("\x1b[103m"),
      bright_blue_bg: String::from("\x1b[104m"),
      bright_magenta_bg: String::from("\x1b[105m"),
      bright_cyan_bg: String::from("\x1b[106m"),
      bright_white_bg: String::from("\x1b[107m"),
    }
  }
}