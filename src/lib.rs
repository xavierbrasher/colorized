pub mod color_definations;
pub mod commands;


#[cfg(test)]
mod tests {
  
    use crate::color_definations::*;
    use crate::commands::*;
    #[test]
    fn test_bg() {
        let bg: ColorsBg = ColorsBg::new();
        assert_eq!(bg.reset_bg, "\x1b[0m");
        assert_eq!(bg.blue_bg, "\x1b[44m");
    }
    
    #[test]
    fn test_fg() {
      let fg: ColorsFg = ColorsFg::new();
      assert_eq!(fg.reset_fg, "\x1b[0m");
      assert_eq!(fg.bright_red_fg, "\x1b[91m")
    }

    #[test]
    fn test_colorized_output() {
      let bg: ColorsBg = ColorsBg::new();
      assert_eq!(colorize_this("wow this is cool", bg.blue_bg), "\x1b[44mwow this is cool\x1b[0m")
    }

    #[test]
    fn test_colorized_printing() {
      let fg: ColorsFg = ColorsFg::new();
      colorize_print("This has no new line ", fg.cyan_fg);
      colorize_println("this does have a new line char", fg.bright_magenta_fg)
    }
}
