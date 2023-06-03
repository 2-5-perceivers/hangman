#[inline]
pub fn get_char_pressed() -> Option<char> {
  let char_code = unsafe { raylib::ffi::GetCharPressed() };
  if char_code > 0 {
    return char::from_u32(char_code as u32);
  }
  None
}
