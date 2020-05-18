// use std::io::prelude::*;

// mod opcode;

// #[derive(Debug, PartialEq)]
// pub enum IR {
//     SHR(u32),
//     SHL(u32),
//     ADD(u8),
//     SUB(u8),
//     PUTCHAR,
//     GETCHAR,
//     JIZ(u32),
//     JNZ(u32),
// }

// fn cast(code: Vec<opcode::Opcode>) -> Vec<IR> {
//     let mut outa: Vec<IR> = Vec::new();
//     let mut jstack: Vec<u32> = Vec::new();
//     for (i, e) in code.iter().enumerate() {
//         match e {
//             opcode::Opcode::SHR => match outa.last_mut() {
//                 Some(IR::SHR(x)) => {
//                     *x += 1;
//                 }
//                 _ => {
//                     outa.push(IR::SHR(1));
//                 }
//             },
//             opcode::Opcode::SHL => match outa.last_mut() {
//                 Some(IR::SHL(x)) => {
//                     *x += 1;
//                 }
//                 _ => {
//                     outa.push(IR::SHL(1));
//                 }
//             },
//             opcode::Opcode::ADD => match outa.last_mut() {
//                 Some(IR::ADD(x)) => {
//                     let (b, _) = x.overflowing_add(1);
//                     *x = b;
//                 }
//                 _ => {
//                     outa.push(IR::ADD(1));
//                 }
//             },
//             opcode::Opcode::SUB => match outa.last_mut() {
//                 Some(IR::SUB(x)) => {
//                     let (b, _) = x.overflowing_add(1);
//                     *x = b;
//                 }
//                 _ => {
//                     outa.push(IR::SUB(1));
//                 }
//             },
//             opcode::Opcode::GETCHAR => {
//                 outa.push(IR::GETCHAR);
//             }
//             opcode::Opcode::PUTCHAR => {
//                 outa.push(IR::PUTCHAR);
//             }
//             opcode::Opcode::LB => {
//                 outa.push(IR::JIZ(0));
//                 jstack.push(i as u32);
//             }
//             opcode::Opcode::RB => {
//                 let j = jstack.pop().ok_or("pop from empty list").unwrap();
//                 outa.push(IR::JNZ(j));
//                 match &mut outa[j as usize] {
//                     IR::JIZ(x) => {
//                         *x = i as u32;
//                     }
//                     _ => {
//                         panic!("");
//                     }
//                 }
//             }
//         }
//     }
//     outa
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = std::env::args().collect();
//     assert!(args.len() >= 2);
//     let mut f = std::fs::File::open(&args[1])?;
//     let mut c: Vec<u8> = Vec::new();
//     f.read_to_end(&mut c)?;

//     let opcode_code = opcode::cast(c);
//     let ir_code = cast(opcode_code);
//     println!("{:?}", ir_code);
//     Ok(())
// }

fn main() {
    println!("Hello");
}
