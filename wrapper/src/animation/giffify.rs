use gif::{Frame, Encoder, Repeat};
use std::borrow::Cow;

pub fn giffify(states: &Vec<Vec<u8>>, width: u16, height: u16) -> Option<Vec<u8>> {
  let color_map = &[0, 0, 0, 0, 0x88, 0xFF, 0x88, 0x88, 0x88];
  let mut image = Vec::new();
  {
      let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
      encoder.set_repeat(Repeat::Infinite).unwrap();
      for state in states {
          let mut frame = Frame::default();
          frame.width = width;
          frame.height = height;
          frame.buffer = Cow::Borrowed(state);
          encoder.write_frame(&frame).unwrap();
      }
  }

  Some(image)
}