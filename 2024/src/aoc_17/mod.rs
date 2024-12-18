use crate::utils::InputMode;
use std::fs;

const TEXT_INPUT: &str = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

#[derive(Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ptr: usize,
    program: Vec<u8>,
    output: Vec<u8>,
    halted: bool,
}

impl Computer {
    fn step(&mut self) {
        let opcode = self.program.get(self.ptr);
        let operand = self.program.get(self.ptr + 1);
        if opcode.is_none() || operand.is_none() {
            self.halted = true;
            return;
        }
        let opcode = opcode.unwrap();
        let operand = *operand.unwrap() as i64;
        let combo = match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => operand,
            _ => panic!("invalid combo"),
        };

        fn div(numerator: i64, operand: i64) -> i64 {
            //2_i64.pow(operand.try_into().unwrap()) => (1 << operand)
            numerator / (1 << operand)
        }

        //println!("opcode: {opcode}, operand: {operand}");
        match opcode {
            0 => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                // The result of the division operation is truncated to an integer and then written to the A register.
                //println!("adv : division into a ");
                self.a = div(self.a, combo);
            }
            1 => {
                //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
                //then stores the result in register B.
                //println!("bxl : xor");
                self.b = self.b ^ operand;
            }
            2 => {
                //The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                //(thereby keeping only its lowest 3 bits), then writes that value to the B register.
                //println!("bst : modulo {operand}");
                self.b = combo % 8;
            }
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However,
                // if the A register is not zero, it jumps by setting the instruction pointer to the value
                // of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                //println!("jnz : jumps");
                if self.a == 0 {
                    self.ptr += 2;
                    return;
                }
                let combo: usize = combo.try_into().unwrap();
                self.ptr = combo;
                return;
            }
            4 => {
                //The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
                //then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                //println!("bxc : bitwise of b and c a");
                self.b = self.b ^ self.c;
            }
            5 => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo
                // 8, then outputs that value. (If a program outputs multiple values, they are
                // separated by commas.)
                //println!("out : modulo");
                let v: u8 = (combo % 8).try_into().unwrap();
                self.output.push(v);
            }
            6 => {
                //The bdv instruction (opcode 6) works exactly like the adv instruction except that the result
                //is stored in the B register. (The numerator is still read from the A register.)
                //println!("bdv : adv into b");
                self.b = div(self.a, combo);
            }
            7 => {
                //The cdv instruction (opcode 7) works exactly like the adv instruction except that the
                //result is stored in the C register. (The numerator is still read from the A register.)
                //println!("cdv : adv into c");
                self.c = div(self.a, combo);
            }
            _ => panic!("Unknown instruction"),
        }

        self.ptr += 2;
    }
}

fn parse(data: &str) -> Computer {
    let (registers, program) = data.trim().split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| line.trim()[11..].trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let program = program[8..]
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    Computer {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        ptr: 0,
        program,
        output: vec![],
        halted: false,
    }
}

fn part1(data: String) -> String {
    let mut computer = parse(&data);
    //let mut computer = Computer {
    //    a: 2024,
    //    b: 0,
    //    c: 0,
    //    ptr: 0,
    //    program: vec![0, 1, 5, 4, 3, 0],
    //    output: vec![],
    //    halted: false,
    //};
    println!("{computer:?}");

    while !computer.halted {
        computer.step();
        //println!("{computer:?}");
    }
    println!("{computer:?}");

    computer
        .output
        .iter()
        .map(|x| format!("{x}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(data: String) -> usize {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_17/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
