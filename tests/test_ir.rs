use brainfuck;

#[test]
fn fold_1() {
    let code_text = "[+++++]".as_bytes();
    let code_op = brainfuck::opcode::Code::from(code_text.to_vec()).unwrap();
    let code_ir = brainfuck::ir::Code::from(code_op.instrs).unwrap();
    assert_eq!(
        code_ir.instrs,
        vec![
            brainfuck::ir::IR::JIZ(2),
            brainfuck::ir::IR::ADD(5),
            brainfuck::ir::IR::JNZ(0)
        ]
    )
}

#[test]
fn fold_2() {
    let code_text = "+[,.]".as_bytes();
    let code_op = brainfuck::opcode::Code::from(code_text.to_vec()).unwrap();
    let code_ir = brainfuck::ir::Code::from(code_op.instrs).unwrap();
    assert_eq!(
        code_ir.instrs,
        vec![
            brainfuck::ir::IR::ADD(1),
            brainfuck::ir::IR::JIZ(4),
            brainfuck::ir::IR::GETCHAR,
            brainfuck::ir::IR::PUTCHAR,
            brainfuck::ir::IR::JNZ(1),
        ]
    )
}
