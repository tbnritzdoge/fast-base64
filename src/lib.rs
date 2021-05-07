#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod statics;
use statics::*;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ErrorKind {
  InvalidCodedCharacter,
  InvalidPaddingCharacter,
  InvalidPaddingLength,
  TrailingSixBits,
  TrailingUnPaddedBits,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Error {
  pos: usize,
  byte: u8,
  kind: ErrorKind,
}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self.kind {
      ErrorKind::InvalidCodedCharacter => {
        write!(
          f,
          "invalid character `{}`({:#x}) at input postion {}",
          self.byte as char, self.byte, self.pos
        )
      }
      ErrorKind::InvalidPaddingCharacter => {
        write!(
          f,
          "invalid padding character `{}`({:#x}) at input postion {}",
          self.byte as char, self.byte, self.pos
        )
      }
      _ => {
        write!(f, "invalid data")
      }
    }
  }
}
use napi::{CallContext, Error as JsError, JsBuffer, JsObject, JsString, Result as JsResult};
impl std::error::Error for Error {}

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_arch = "aarch64"),
  not(target_arch = "arm"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> JsResult<()> {
  exports.create_named_method("decode", js_decode)?;
  exports.create_named_method("encode", js_encode)?;
  Ok(())
}
#[js_function(1)]
fn js_decode(ctx: CallContext) -> JsResult<JsString> {
  let argument: Vec<u8> = ctx.get::<JsBuffer>(0)?.into_value()?.to_vec();
  let returned = decode(argument);
  match returned {
    Ok(value) => {
      let string = unsafe { std::str::from_utf8_unchecked(&value) };
      ctx.env.create_string(&string)
    }
    Err(err) => Err(JsError::from_reason(format!("{:?}", err))),
  }
}

#[js_function(1)]
fn js_encode(ctx: CallContext) -> JsResult<JsString> {
  let input: Vec<u8> = ctx.get::<JsBuffer>(0)?.into_value()?.to_vec();
  if input.is_empty() {
    return ctx.env.create_string("");
  }

  let mut output = vec![b'='; encode_buffer_len(input.len())];

  encode_to_slice(input, &mut output);
  ctx
    .env
    .create_string(unsafe { std::str::from_utf8_unchecked(&output) })
}

#[inline]
pub fn decode_buffer_len(ilen: usize) -> usize {
  let n = ilen / 4;
  let r = ilen % 4;

  let olen = if r > 0 { n * 3 + 3 } else { n * 3 };

  olen
}

#[inline]
pub fn encode_buffer_len(ilen: usize) -> usize {
  let n = ilen / 3;
  let r = ilen % 3;
  let olen = if r > 0 { n * 4 + 4 } else { n * 4 };

  olen
}

pub fn decode<D: AsRef<[u8]>>(input: D) -> Result<Vec<u8>, Error> {
  let input = input.as_ref();
  if input.is_empty() {
    return Ok(Vec::new());
  }

  let ilen = input.len();
  let olen = decode_buffer_len(ilen);

  let mut output = vec![0u8; olen];

  let amt = decode_to_slice(input, &mut output)?;
  if amt < olen {
    output.truncate(amt);
  }

  Ok(output)
}

#[inline]
pub fn decode_to_slice<R: AsRef<[u8]>, W: AsMut<[u8]>>(
  input: R,
  output: &mut W,
) -> Result<usize, Error> {
  decode_to_slice_inner(&DECODE_TABLE, input, output)
}

#[inline]
fn decode_to_slice_inner<R: AsRef<[u8]>, W: AsMut<[u8]>>(
  table: &[u8; 256],
  input: R,
  output: &mut W,
) -> Result<usize, Error> {
  let input = input.as_ref();
  let output = output.as_mut();

  let ilen = input.len();

  let mut ipos = 0usize;
  let mut opos = 0usize;
  let mut group = 0u32;
  let mut gpos = 0u8;
  let mut plen = 0usize;

  while ipos < ilen {
    let val = table[input[ipos] as usize];
    match val {
      ____ => {
        return Err(Error {
          pos: ipos,
          byte: input[ipos],
          kind: ErrorKind::InvalidCodedCharacter,
        });
      }
      _EXT => {
        const MAX_PADDING_LEN: usize = 2;

        plen = 1;
        ipos += 1;

        while ipos < ilen {
          let val = table[input[ipos] as usize];
          if val != _EXT || plen >= MAX_PADDING_LEN {
            return Err(Error {
              pos: ipos,
              byte: input[ipos],
              kind: if val != _EXT {
                ErrorKind::InvalidPaddingCharacter
              } else {
                ErrorKind::InvalidPaddingLength
              },
            });
          }

          ipos += 1;
          plen += 1;
        }

        if ilen % 4 > 0 {
          return Err(Error {
            pos: ipos,
            byte: input[ipos - 1],
            kind: ErrorKind::InvalidPaddingLength,
          });
        }

        break;
      }
      _ => {
        match gpos {
          0 => {
            group = (val as u32) << 26;
            gpos = 6;
          }
          6 => {
            group |= (val as u32) << 20;
            gpos = 12;
          }
          12 => {
            group |= (val as u32) << 14;
            gpos = 18;
          }
          18 => {
            group |= (val as u32) << 8;
            let [b1, b2, b3, _] = group.to_be_bytes();

            output[opos + 0] = b1;
            output[opos + 1] = b2;
            output[opos + 2] = b3;

            opos += 3;
            gpos = 0;
          }
          _ => unreachable!(),
        }

        ipos += 1;
      }
    }
  }

  match gpos {
    0 => {}
    6 => {
      ipos -= 1;
      return Err(Error {
        pos: ipos,
        byte: input[ipos],
        kind: ErrorKind::TrailingSixBits,
      });
    }
    12 => {
      let [b1, _, _, _] = group.to_be_bytes();

      output[opos + 0] = b1;

      opos += 1;

      if plen != 2 {
        ipos -= 1;
        return Err(Error {
          pos: ipos,
          byte: input[ipos],
          kind: ErrorKind::TrailingUnPaddedBits,
        });
      }
    }
    18 => {
      let [b1, b2, _, _] = group.to_be_bytes();

      output[opos + 0] = b1;
      output[opos + 1] = b2;

      opos += 2;

      if plen != 1 {
        ipos -= 1;
        return Err(Error {
          pos: ipos,
          byte: input[ipos],
          kind: ErrorKind::TrailingUnPaddedBits,
        });
      }
    }
    _ => unreachable!(),
  }

  Ok(opos)
}

#[inline]
pub fn encode_to_slice<D: AsRef<[u8]>, W: AsMut<[u8]>>(input: D, output: &mut W) {
  encode_to_slice_inner(&TABLE, input, output);
}

#[inline]
fn encode_to_slice_inner<R: AsRef<[u8]>, W: AsMut<[u8]>>(
  table: &[u8; 64],
  input: R,
  output: &mut W,
) {
  let input = input.as_ref();
  let output = output.as_mut();

  let ilen = input.len();
  let n = ilen / 3;
  let r = ilen % 3;

  let mut i = 0usize;
  while i < n {
    let num = u32::from_be_bytes([input[i * 3 + 0], input[i * 3 + 1], input[i * 3 + 2], 0]);

    output[i * 4 + 0] = table[((num >> 26) & 0x3F) as usize];
    output[i * 4 + 1] = table[((num >> 20) & 0x3F) as usize];
    output[i * 4 + 2] = table[((num >> 14) & 0x3F) as usize];
    output[i * 4 + 3] = table[((num >> 8) & 0x3F) as usize];

    i += 1;
  }
  match r {
    0 => {}
    1 => {
      let num = u32::from_be_bytes([input[i * 3 + 0], 0, 0, 0]);

      output[i * 4 + 0] = table[((num >> 26) & 0x3F) as usize];
      output[i * 4 + 1] = table[((num >> 20) & 0x3F) as usize];
    }
    2 => {
      let num = u32::from_be_bytes([input[i * 3 + 0], input[i * 3 + 1], 0, 0]);

      output[i * 4 + 0] = table[((num >> 26) & 0x3F) as usize];
      output[i * 4 + 1] = table[((num >> 20) & 0x3F) as usize];
      output[i * 4 + 2] = table[((num >> 14) & 0x3F) as usize];
    }
    _ => unreachable!(),
  }
}
