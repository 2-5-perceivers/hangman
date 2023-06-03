use raylib::RaylibHandle;

pub struct WindowSize {
  pub width: i32,
  pub height: i32,
}

impl WindowSize {
  pub fn new(w: i32, h: i32) -> WindowSize {
    Self { width: w, height: h }
  }

  pub fn update(&mut self, rl: &RaylibHandle) {
    self.height = rl.get_screen_height();
    self.width = rl.get_screen_width();
  }
}
