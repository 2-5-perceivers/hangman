use raylib::{
  prelude::{Color, MouseButton, RaylibDraw, RaylibDrawHandle, Rectangle},
  text::measure_text,
};

use crate::window_size::WindowSize;

pub fn anchor_pos(
  w: &WindowSize,
  a_x: f32,
  a_y: f32,
  o_x: f32,
  o_y: f32,
  s_x: f32,
  s_y: f32,
) -> Rectangle {
  Rectangle {
    x: (w.width as f32 * a_x) + o_x - (s_x / 2.0),
    y: (w.height as f32 * a_y) + o_y - (s_y / 2.0),
    width: s_x,
    height: s_y,
  }
}

pub fn draw_button(
  d: &mut RaylibDrawHandle,
  w: &WindowSize,
  text: &str,
  font_size: i32,
  a_x: f32,
  a_y: f32,
  color: Color,
) -> bool {
  let pressed: bool;

  let mut color: Color = color;

  let text_layer: Rectangle = anchor_pos(
    w,
    a_x,
    a_y,
    0.0,
    0.0,
    measure_text(text, font_size) as f32,
    font_size as f32,
  );
  let button_layer: Rectangle = anchor_pos(
    w,
    a_x,
    a_y,
    0.0,
    0.0,
    text_layer.width + 32.0,
    text_layer.height + 16.0,
  );

  let hovered: bool =
    button_layer.check_collision_point_rec(d.get_mouse_position());

  if hovered {
    pressed = d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON);
  } else {
    pressed = false;
  }

  if hovered {
    color.r = (color.r as f32 * 0.7) as u8;
    color.g = (color.g as f32 * 0.7) as u8;
    color.b = (color.b as f32 * 0.7) as u8;
  }

  d.draw_rectangle_rounded_lines(button_layer, 0.5, 0, 2, color);
  d.draw_text(text, text_layer.x as i32, text_layer.y as i32, font_size, color);

  pressed
}
