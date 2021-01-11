# Rustboy
## Gameboy Emulator Written in Rust
* I am mainly making this to learn more about old architectures and as a fun learning project to get better as Rust. Not neccessarily the most accurate/correct emulation of the GameBoy.
## Progress
### 1/10/21
* Most of the main opcodes have been added
* Just the EI/DI and the remaining CB opcodes are left
* Testing to make sure the instructions work as intended remains as well

### 9/15/20
* instructions.rs contains most of the decoding of the assembly and the corresponding loads/stores
* cpu.rs contains the execution loop of the cpu and the reg files/memory layout
  * Gameboy uses a Fetch/Execute overlap so certain instructions can fetch and execute in parallel across a clock cycle
## Sources
https://realboyemulator.files.wordpress.com/2013/01/gbcpuman.pdf

https://gekkio.fi/files/gb-docs/gbctr.pdf

https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8
