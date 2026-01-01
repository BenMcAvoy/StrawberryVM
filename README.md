<div align="center">
  <a href="https://github.com/BenMcAvoy/StrawberryVM">
    <img src="assets/logo-128x128.png" alt="Logo" width="128" height="128">
  </a>

  <h3 align="center">StrawberryVM</h3>

  <p align="center">
    A fantasy virtual machine with limits on resources.
    <br />
    <a href="https://docs.rs/strawberryvm"><strong>« Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/BenMcAvoy/StrawberryVM/releases">Releases</a>
    ·
    <a href="https://github.com/BenMcAvoy/StrawberryVM/issues">Report Bugs</a>
    ·
    <a href="https://github.com/BenMcAvoy/StrawberryVM/issues">Request Features</a>
  </p>
</div>



## TODO
- [x] Basic instructions
- [x] Addition implementation
- [x] Loading from a file
- [x] Text to binary
- [x] Assembler
- [x] Refactoring
- [x] Turing completion
- [ ] TCC support

## Instruction Set

| Opcode | Name | Arguments | Description |
|--------|------|-----------|-------------|
| `0x00` | `Nop` | None | No operation. |
| `0x10` | `Push` | `u8` | Pushes an 8-bit immediate onto the stack. |
| `0x11` | `Pop` | `Register` | Pops the top of the stack into the specified register. |
| `0x12` | `PushReg` | `Register` | Pushes the value of the specified register onto the stack. |
| `0x13` | `Mov` | `Register, Register` | Copies the value from the second register to the first. |
| `0x20` | `Add` | `Register, Register` | Adds the second register to the first and stores the result in the first. |
| `0x21` | `Sub` | `Register, Register` | Subtracts the second register from the first and stores the result in the first. |
| `0x22` | `Shl` | `Register, Register` | Shifts left the first register by the amount contained in the second. |
| `0x23` | `Shr` | `Register, Register` | Shifts right the first register by the amount contained in the second. |
| `0x24` | `And` | `Register, Register` | Bitwise AND between registers, result in first. |
| `0x25` | `Or` | `Register, Register` | Bitwise OR between registers, result in first. |
| `0x26` | `Xor` | `Register, Register` | Bitwise XOR between registers, result in first. |
| `0x27` | `Not` | `Register` | Bitwise NOT on a register. |
| `0x28` | `Mul` | `Register, Register` | Multiply two registers; result in first. |
| `0x29` | `Div` | `Register, Register` | Divide two registers; result in first. |
| `0x30` | `Cmp` | `Register, Register` | Compare two registers and set flags (Compare, Negative, Overflow). |
| `0x31` | `Jmp` | `i8` | Jump by signed offset (in instructions / 16-bit words) relative to the instruction pointer. |
| `0x32` | `Je` | `i8` | Jump by signed offset (in instructions / 16-bit words) if last compare indicated equality. |
| `0x33` | `Jne` | `i8` | Jump by signed offset (in instructions / 16-bit words) if last compare indicated inequality. |
| `0x40` | `Load` | `Register, Register` | Load a `u16` from memory at address in second register into first. |
| `0x41` | `Store` | `Register, Register` | Store a `u16` from first register into memory at address in second. |
| `0x50` | `Signal` | `u8` | Host call sending an 8-bit signal value (used for logging / halting). |

> Note: `Push` and `Signal` accept 8-bit immediates. `Jmp`/`Je`/`Jne` take signed 8-bit offsets measured in instructions (each instruction is 16 bits / 2 bytes). Instruction encoding packs the opcode in the low byte and operands in the high byte(s).

### Registers

- General purpose: `A`, `B`, `C`, `D`
- Special: `SP` (stack pointer), `PC` (program counter), `BP` (base pointer), `FL` (flags)

Flags (in `FL`):
- `Compare` = 1 (set when equal)
- `Negative` = 2 (set on negative result)
- `Overflow` = 4 (set on arithmetic overflow)

---

## Reserved symbols
| Symbol | Use               |
|--------|-------------------|
| $      | Hexadecimal value |
| %      | Binary value      |
| ^      | Label value       |

## Credits
This project is following along with the live streams by [TomMarksTalksCode](https://www.youtube.com/@TomMarksTalksCode) and this project would not have been possible without him. He can also be found on [GitHub](https://github.com/phy1um) and on his [website](https://coding.tommarks.xyz/).
