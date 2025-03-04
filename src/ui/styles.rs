use ratatui::{
  prelude::Color,
  style::{Styled, Stylize},
};

pub const COLOR_WHITE: Color = Color::Rgb(248, 248, 242);
pub const COLOR_GREEN: Color = Color::Rgb(80, 250, 123);
pub const COLOR_MAGENTA: Color = Color::Rgb(189, 147, 249);
pub const COLOR_RED: Color = Color::Rgb(255, 85, 85);
pub const COLOR_BLUE: Color = Color::Rgb(98, 114, 164);
pub const COLOR_CYAN: Color = Color::Rgb(189, 147, 249);
pub const COLOR_GREY: Color = Color::Rgb(68, 71, 90);
pub const COLOR_YELLOW: Color = Color::Rgb(241, 250, 140);
pub const COLOR_ORANGE: Color = Color::Rgb(255, 184, 108);

#[cfg(test)]
#[path = "styles_tests.rs"]
mod styles_tests;

pub trait ManagarrStyle<'a, T>: Stylize<'a, T>
where
  T: Default,
{
  #[allow(clippy::new_ret_no_self)]
  fn new() -> T;
  fn awaiting_import(self) -> T;
  fn indeterminate(self) -> T;
  fn default(self) -> T;
  fn downloaded(self) -> T;
  fn downloading(self) -> T;
  fn failure(self) -> T;
  fn help(self) -> T;
  fn highlight(self) -> T;
  fn missing(self) -> T;
  fn primary(self) -> T;
  fn secondary(self) -> T;
  fn success(self) -> T;
  fn system_function(self) -> T;
  fn unmonitored(self) -> T;
  fn unmonitored_missing(self) -> T;
  fn unreleased(self) -> T;
  fn warning(self) -> T;
}

impl<T, U> ManagarrStyle<'_, T> for U
where
  U: Styled<Item = T>,
  T: Default,
{
  fn new() -> T {
    T::default()
  }

  fn awaiting_import(self) -> T {
    self.fg(COLOR_ORANGE)
  }

  fn indeterminate(self) -> T {
    self.fg(COLOR_ORANGE)
  }

  fn default(self) -> T {
    self.fg(COLOR_WHITE)
  }

  fn downloaded(self) -> T {
    self.fg(COLOR_GREEN)
  }

  fn downloading(self) -> T {
    self.fg(COLOR_MAGENTA)
  }

  fn failure(self) -> T {
    self.fg(COLOR_RED)
  }

  fn help(self) -> T {
    self.fg(COLOR_BLUE)
  }

  fn highlight(self) -> T {
    self.reversed()
  }

  fn missing(self) -> T {
    self.fg(COLOR_RED)
  }

  fn primary(self) -> T {
    self.fg(COLOR_CYAN)
  }

  fn secondary(self) -> T {
    self.fg(COLOR_YELLOW)
  }

  fn success(self) -> T {
    self.fg(COLOR_GREEN)
  }

  fn system_function(self) -> T {
    self.fg(COLOR_YELLOW)
  }

  fn unmonitored(self) -> T {
    self.fg(COLOR_GREY)
  }

  fn unmonitored_missing(self) -> T {
    self.fg(COLOR_YELLOW)
  }

  fn unreleased(self) -> T {
    self.fg(COLOR_CYAN)
  }

  fn warning(self) -> T {
    self.fg(COLOR_MAGENTA)
  }
}
