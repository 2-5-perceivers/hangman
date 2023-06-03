use raylib::{
  get_random_value,
  prelude::{KeyboardKey, RaylibDraw, RaylibDrawHandle},
  text::measure_text,
};

use crate::{
  colors,
  gui::{anchor_pos, draw_button},
  utils::get_char_pressed,
  window_size::WindowSize,
  words::Words,
};

pub enum GameState {
  START,
  RUN(i32, Option<char>),
  LOST,
  WON,
  FINISHED,
}

pub fn draw_title(d: &mut RaylibDrawHandle) {
  d.draw_text("Spanzuratoarea", 32, 32, 40, colors::PRIMARY);
  d.draw_text("Un proiect de Vanca Rares", 32, 32 + 40 + 4, 20, colors::TEXT);
}

pub fn draw_start(d: &mut RaylibDrawHandle, w: &WindowSize, g: &mut GameState) {
  let button_pressed =
    draw_button(d, w, "Incepe", 30, 0.5, 0.5, colors::PRIMARY);

  if button_pressed {
    *g = GameState::RUN(0, None);
  }
}

pub fn draw_run(
  d: &mut RaylibDrawHandle,
  w: &WindowSize,
  g: &mut GameState,
  words: &mut Words,
) {
  let mut will_close = 0;

  {
    let GameState::RUN(incercari_ramase, ultima_incercare) = g else {panic!()};

    if words.processed_word.is_none() {
      let chosen_word_index: i32 =
        get_random_value(0, (words.words.len() - 1) as i32);

      words.process_word(chosen_word_index as usize);

      *incercari_ramase = 5;
      *ultima_incercare = None;
    }

    if let Some(pw) = &mut words.processed_word {
      // Desenam incercarile
      {
        let incercari = "Incercari";
        let numar = format!("{}", incercari_ramase);

        let incercari_rect = anchor_pos(
          w,
          0.8,
          0.5,
          0.0,
          0.0,
          measure_text(incercari, 30) as f32,
          60.0,
        );

        d.draw_text(
          incercari,
          incercari_rect.x as i32,
          incercari_rect.y as i32,
          30,
          colors::TEXT,
        );
        d.draw_text(
          numar.as_str(),
          incercari_rect.x as i32,
          (incercari_rect.y as i32) + 30,
          30,
          colors::PRIMARY,
        );
      }

      // Desenam cuvantul
      {
        let cuv_rect = anchor_pos(
          w,
          0.5,
          0.5,
          0.0,
          0.0,
          measure_text(pw.visible.as_str(), 60) as f32,
          60.0,
        );

        d.draw_text(
          pw.visible.as_str(),
          cuv_rect.x as i32,
          cuv_rect.y as i32,
          60,
          colors::PRIMARY,
        );
      }

      // Desenam ultima incercare
      {
        let key = get_char_pressed();
        match key {
          Some(char) => {
            let try_result = pw.try_char(&char);
            match try_result {
              crate::words::LetterTryResult::NUP => {
                *ultima_incercare = Some(char);
                *incercari_ramase -= 1;

                if *incercari_ramase == 0 {
                  will_close = 1;
                }
              }
              crate::words::LetterTryResult::GOOD => {
                *ultima_incercare = None;
              }
              crate::words::LetterTryResult::FINISHED => {
                will_close = 2;
              }
            }
          }
          None => {}
        }

        if let Some(uinc) = ultima_incercare {
          let nu_este = "Nu este in cuvant";

          let ultima_incercare_rect = anchor_pos(
            w,
            0.5,
            0.9,
            0.0,
            0.0,
            measure_text(nu_este, 20) as f32,
            20.0,
          );

          d.draw_text(
            format!("{}", uinc).as_str(),
            ultima_incercare_rect.x as i32,
            ultima_incercare_rect.y as i32,
            20,
            colors::TEXT,
          );
          d.draw_text(
            nu_este,
            ultima_incercare_rect.x as i32,
            ultima_incercare_rect.y as i32 + 20,
            20,
            colors::PRIMARY,
          );
        }
      }
    }
  }

  match will_close {
    1 => *g = GameState::LOST,
    2 => {
      if words.words.is_empty() {
        *g = GameState::FINISHED;
      } else {
        *g = GameState::WON;
      }
    }
    _ => {}
  }
}

pub fn draw_won(
  d: &mut RaylibDrawHandle,
  w: &WindowSize,
  g: &mut GameState,
  words: &mut Words,
) {
  // Desenam felicitarile
  {
    let felicitari = "Felicitari";
    let felicitari_rect = anchor_pos(
      w,
      0.8,
      0.5,
      0.0,
      0.0,
      measure_text(felicitari, 30) as f32,
      30.0,
    );

    d.draw_text(
      felicitari,
      felicitari_rect.x as i32,
      felicitari_rect.y as i32,
      30,
      colors::PRIMARY,
    );
  }

  // Desenam mare cuvantul
  {
    let word = words.processed_word.as_ref().unwrap().visible.as_str();

    let word_rect =
      anchor_pos(w, 0.5, 0.5, 0.0, 0.0, measure_text(word, 100) as f32, 100.0);
    d.draw_text(
      word,
      word_rect.x as i32,
      word_rect.y as i32,
      100,
      colors::TEXT,
    );
  }

  // Butonul de urmatorul
  {
    let pressed = draw_button(d, w, "Urmatorul", 30, 0.5, 0.8, colors::PRIMARY)
      || d.is_key_released(KeyboardKey::KEY_ENTER)
      || d.is_key_released(KeyboardKey::KEY_SPACE);

    if pressed {
      words.processed_word = None;
      *g = GameState::RUN(0, None);
    }
  }
}

pub fn draw_lost(d: &mut RaylibDrawHandle, w: &WindowSize) {
  let end = "AI PIERDUT";
  let fontsize = 100;

  let end_rect = anchor_pos(
    w,
    0.5,
    0.5,
    0.0,
    0.0,
    measure_text(end, fontsize) as f32,
    fontsize as f32,
  );
  d.draw_text(
    end,
    end_rect.x as i32,
    end_rect.y as i32,
    fontsize,
    colors::PRIMARY,
  );
}

pub fn draw_finished(d: &mut RaylibDrawHandle, w: &WindowSize) {
  let end = "AI TERMINAT\nTOATE CUVINTELE";
  let fontsize = 80;

  let end_rect = anchor_pos(
    w,
    0.5,
    0.5,
    0.0,
    0.0,
    measure_text(end, fontsize) as f32,
    (fontsize * 2) as f32,
  );
  d.draw_text(
    end,
    end_rect.x as i32,
    end_rect.y as i32,
    fontsize,
    colors::TEXT,
  );
}
