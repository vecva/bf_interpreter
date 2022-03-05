macro_rules! COMMANDS {
  () => {
    "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
  };
}

const COMMANDS: &[u8] = COMMANDS!().as_bytes();
const COMMANDS_LENGTH: usize = COMMANDS!().len();

fn get_jumps() -> std::collections::HashMap<usize, usize> {
  let mut vector = Vec::new();
  let mut jumps = std::collections::HashMap::new();

  for (index, command) in COMMANDS.iter().enumerate() {
    if command.eq(&b'[') {
      vector.push(index);
    } else if command.eq(&b']') {
      let start_index = vector.pop().unwrap();
      jumps.insert(index, start_index);
      jumps.insert(start_index, index);
    }
  }

  jumps
}

fn main() {
  let jumps = get_jumps();
  let mut data = Vec::new();
  data.push(0);
  let mut data_pointer = 0;
  let mut instruction_pointer = 0;

  loop {
    match COMMANDS[instruction_pointer] {
      b'+' => data[data_pointer] += 1,
      b',' => {
        use std::io::Read;
        data[data_pointer] = std::io::stdin().bytes().nth(0).unwrap().unwrap();
      }
      b'-' => data[data_pointer] -= 1,
      b'.' => print!("{}", data[data_pointer] as char),
      b'<' => data_pointer -= 1,
      b'>' => {
        data_pointer += 1;

        if data_pointer > data.len() - 1 {
          data.push(0);
        }
      }
      b'[' => {
        if data[data_pointer].eq(&0) {
          instruction_pointer = *jumps.get(&instruction_pointer).unwrap();
        }
      }
      b']' => {
        if data[data_pointer].ne(&0) {
          instruction_pointer = *jumps.get(&instruction_pointer).unwrap();
        }
      }
      _ => {}
    }

    instruction_pointer += 1;

    if instruction_pointer > COMMANDS_LENGTH - 1 {
      return;
    }
  }
}