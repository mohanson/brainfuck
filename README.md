# Brainfuck

Brainfuck is an esoteric programming language created in 1993 by Urban Müller, and notable for its extreme minimalism.
The language consists of only eight simple commands and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers. Brainfuck simply requires one to break commands into microscopic steps.

The language's name is a reference to the slang term brainfuck, which refers to things so complicated or unusual that they exceed the limits of one's understanding.

This project provides three ways to execute BF code:

- Raw interpreter without any trick
- Optimized IR
- JIT

# Raw interpreter without any trick

```
$ cargo run --release --bin brainfuck_interpreter ./res/mandelbrot.bf
```

![img](/res/mandelbrot_interpreter.gif)

# Optimized IR

```
$ cargo run --release --bin brainfuck_ir ./res/mandelbrot.bf
```

![img](/res/mandelbrot_ir.gif)

# JIT

```
$ cargo run --release --bin brainfuck_jit ./res/mandelbrot.bf
```

![img](/res/mandelbrot_jit.gif)

# Licences

MIT
