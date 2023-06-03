use game_state::{
  draw_finished, draw_lost, draw_run, draw_start, draw_title, draw_won,
};

use raylib::prelude::*;
use window_size::WindowSize;
use words::Words;

mod colors;
mod game_state;
mod gui;
mod utils;
mod window_size;
mod words;

fn main() {
  let mut w = WindowSize::new(1280, 720);
  let mut words = Words::new();

  let mut game_state = game_state::GameState::START;

  let (mut rl, thread) = raylib::init()
    .size(w.width, w.height)
    .title("Spanzuratoarea | Proiect de Vanca Rares")
    .resizable()
    .build();

  //rl.set_target_fps(360);

  while !rl.window_should_close() {
    w.update(&rl);
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(colors::BACKGROUND);
    d.draw_text(format!("{}", d.get_fps()).as_str(), 10, 10, 10, Color::GREEN);

    draw_title(&mut d);

    match game_state {
      game_state::GameState::START => {
        draw_start(&mut d, &w, &mut game_state);
      }
      game_state::GameState::RUN(_, _) => {
        draw_run(&mut d, &w, &mut game_state, &mut words);
      }
      game_state::GameState::LOST => {
        draw_lost(&mut d, &w);
      }
      game_state::GameState::WON => {
        draw_won(&mut d, &w, &mut game_state, &mut words);
      }
      game_state::GameState::FINISHED => {
        draw_finished(&mut d, &w);
      }
    }
  }
}
