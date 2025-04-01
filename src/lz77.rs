use crate::util::{transmute_to_u16, transmute_to_u32};

// A very dumb compression implementation fo LZ77 that will actually *increase* file size, not decrease it.
pub(crate) fn lz77_compress(input: &[u8]) -> Vec<u8> {
  (input.len() as u32)
    .to_le_bytes()
    .into_iter()
    .chain(
      input
        .chunks(8)
        .flat_map(|chunk| [0xFF].iter().chain(chunk))
        .copied(),
    )
    .collect()
}

// This is a 1:1 translation of the python code from https://github.com/mchubby/yetireg_tools/tree/master/splz77/splz77_decompress.py
// Original LZSS decompression code written by Treeki - http://jul.rustedlogic.net/thread.php?pid=413390#413390
// Adapted for use in PS2 Strawberry Panic, and later, Cross Channel.
pub(crate) fn lz77_decompress(input: &[u8]) -> Vec<u8> {
  let size = transmute_to_u32(0, input) as isize;

  let mut output = vec![0; size as usize];

  let mut input_ptr = 4usize;

  let mut offset = 0isize;

  while offset < size {
    if input_ptr >= input.len() {
      break;
    }
    let mut flags = input[input_ptr];
    input_ptr += 1;

    for _ in 0..8 {
      if input_ptr >= input.len() {
        break;
      }

      if flags & 1 == 1 {
        output[offset as usize] = input[input_ptr];
        input_ptr += 1;
        offset += 1;
      } else {
        let info = transmute_to_u16(input_ptr, input);
        input_ptr += 2;

        let n_bytes = 3 + ((info & 0x0F00) >> 8);
        let buffer_offset = (((info & 0xF000) >> 4) | (info & 0xFF)) as isize;
        let mut ptr = offset - ((offset - 18 - buffer_offset) & 0x0FFF);

        for _ in 0..n_bytes {
          output[offset as usize] = if ptr < 0 { 0 } else { output[ptr as usize] };
          offset += 1;
          ptr += 1;
          if offset >= size {
            break;
          }
        }
      }
      flags >>= 1;
      if offset >= size {
        break;
      }
    }
  }

  output
}
