use std::env;
use std::io;
use std::io::Read;
use std::u16;
use std::char;

fn main() {
  let mut args = env::args();
  if args.len() != 2 {
    println!("usage: <expr>");
  }
  else {
    brainfuck(args.nth(1).unwrap());
  }
}

fn brainfuck(text: String) {
  let bytes = text.as_bytes();
  let mut pos: usize = 0;
  let mut pointer: usize = 0;
  let mut values: [u32; u16::MAX as usize] = [0; u16::MAX as usize]; 

  while pos < bytes.len() {
    match bytes[pos] {
      b'>' => {
        pointer += 1;
      },

      b'<' => {
        if pointer > 0 {
          pointer -= 1;
        }
      },

      b'+' => {
        values[pointer] += 1;
      },

      b'-' => {
        values[pointer] -= 1;
      },

      b'.' => {
        if let Some(c) = char::from_u32(values[pointer]) {
          print!("{}", c);
        }
        else {
          print!("?");
        }
      }

      b',' => {
        values[pointer] = io::stdin().bytes().nth(0).unwrap().unwrap() as u32;
      }

      b'[' => {
        if values[pointer] == 0 {
          let mut level = 0;
          let mut p = pos;

          loop {
            p += 1;

            if bytes[p] == b'[' {
              level += 1;
            }
            else if bytes[p] == b']' {
              if level == 0 {
                pos = p;
                break;
              }
              else {
                level -= 1;
              }
            }
          }
        }
      },

      b']' => {
        if values[pointer] != 0 {
          let mut level = 0;
          let mut p = pos;

          loop {
            p -= 1;

            if bytes[p] == b']' {
              level += 1;
            }
            else if bytes[p] == b'[' {
              if level == 0 {
                pos = p;
                break;
              }
              else {
                level -= 1;
              }
            }
          }
        }
      },

      b => {
        println!("brainfuck unexpect {}", b);
        break;
      }
    }

    pos += 1;
  }
}

