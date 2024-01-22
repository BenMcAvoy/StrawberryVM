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
- [ ] Refactoring
- [ ] Turing completion

## Instructions
| Name          | Arguments                                       | Description                                                                         |
|---------------|-------------------------------------------------|-------------------------------------------------------------------------------------|
| No Operation  | None                                            | Does nothing.                                                                       |
| Push          | u8 (8-bit value to push)                        | Pushes an 8-bit value onto the stack.                                               |
| Pop Register  | Register (destination register)                 | Pops a value from the stack into the specified register.                            |
| Push Register | Register (source register)                      | Pushes the value of the specified register onto the stack.                          |
| Add Stack     | None                                            | Adds the top two values on the stack.                                               |
| Add Register  | Two Registers (operands)                        | Adds the values of two registers and stores the result in the destination register. |
| Signal        | u8 (signal value)                               | Sends a signal with an 8-bit value.                                                 |
| Jump          | u8 (target address)                             | Jumps to the specified address in the program.                                      |
| ShiftLeft     | Register (target register)and u8 (shift amount) | Left shifts a specific register by a certain amount.                                |


## Reserved symbols
| Symbol | Use               |
|--------|-------------------|
| $      | Hexadecimal value |
| %      | Binary value      |
| ^      | Label value       |

## Credits
This project is following along with the live streams by [TomMarksTalksCode](https://www.youtube.com/@TomMarksTalksCode) and this project would not have been possible without him. He can also be found on [GitHub](https://github.com/phy1um) and on his [website](https://coding.tommarks.xyz/).
