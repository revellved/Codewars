pub fn _brain_luck(code: &str, mut input: Vec<u8>) -> Vec<u8> {
    const SIZE_MEMORY: usize = 30000;

    let code = code.as_bytes();
    let mut code_index = 0;
    let mut tape_index = 0; // Started on middle. in case the brainfack programmer wants to go to the left ;)
    let mut tape_memory: Vec<u8> = vec![0; SIZE_MEMORY];
    let mut final_output: Vec<u8> = vec![];
    let mut code_loops_stack: Vec<usize> = vec![];

    while code_index < code.len() {
        match code[code_index] as char {
            '.' => final_output.push(tape_memory[tape_index]),
            ',' => tape_memory[tape_index] = input.remove(0),
            '<' => tape_index -= 1,
            '>' => tape_index += 1,
            '+' => tape_memory[tape_index] = tape_memory[tape_index].wrapping_add(1),
            '-' => tape_memory[tape_index] = tape_memory[tape_index].wrapping_sub(1),
            '[' => code_loops_stack.push(code_index),
            ']' => {
                if tape_memory[tape_index] != 0 {
                    code_index = *code_loops_stack.last().unwrap();
                } else {
                    code_loops_stack.pop();
                }
            }
            _ => (),
        };
        code_index += 1;
    }
    final_output
}

pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut data = vec![0u8; 30000];
    let mut data_ptr = 0;
    let mut code_ptr = 0;
    let mut input_ptr = 0;

    while code_ptr < code.len() {
        match code.chars().nth(code_ptr).unwrap() {
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '+' => data[data_ptr] = data[data_ptr].wrapping_add(1),
            '-' => data[data_ptr] = data[data_ptr].wrapping_sub(1),
            '.' => output.push(data[data_ptr]),
            ',' => {
                if input_ptr < input.len() {
                    data[data_ptr] = input[input_ptr];
                    input_ptr += 1;
                }
            }
            '[' => {
                if data[data_ptr] == 0 {
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        code_ptr += 1;
                        if code.chars().nth(code_ptr).unwrap() == '[' {
                            loop_count += 1;
                        } else if code.chars().nth(code_ptr).unwrap() == ']' {
                            loop_count -= 1;
                        }
                    }
                }
            }
            ']' => {
                if data[data_ptr] != 0 {
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        code_ptr -= 1;
                        if code.chars().nth(code_ptr).unwrap() == ']' {
                            loop_count += 1;
                        } else if code.chars().nth(code_ptr).unwrap() == '[' {
                            loop_count -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        code_ptr += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        // Echo until byte 255 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(),
            "Codewars"
        );
        // Echo until byte 0 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(),
            "Codewars"
        );
        // Multiply two numbers
        assert_eq!(
            brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]),
            vec![72]
        );
    }

    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
        let mut v = s.to_string().into_bytes();
        v.push(i);
        v
    }
}
