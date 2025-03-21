# 🧱 RISC-V Emulator Core – Build Plan (Rust)

## 🌟 Goal

Build a minimal RISC-V emulator in Rust to:
- Support the `RV32I` or `RV64I` base instruction set
- Implement a unified memory model
- Handle traps (for syscalls/RPCs)
- Load ELF binaries
- Prepare for integration with a microkernel and ring-buffer-based RPC

---

## 📦 1. Project Setup

```bash
cargo new riscv_emulator --bin
cd riscv_emulator
```

**Suggested project structure:**

```
src/
├── main.rs
├── cpu.rs
├── memory.rs
├── bus.rs
├── elf_loader.rs
├── trap.rs
```

---

## 🤩 2. Define Core Types

- General-purpose registers: `x0`–`x31`
- Program Counter (`pc`)
- CSR registers
- Instruction enums for RV32I formats

**`cpu.rs` Example:**

```rust
pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub csr: [u32; 4096],
}
```

---

## 🧠 3. Instruction Decoding

- Parse 32-bit instructions into enum variants
- Extract fields like opcode, `rd`, `rs1`, `rs2`, `imm`

```rust
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Lw { rd: usize, rs1: usize, imm: i32 },
    // ...
}
```

---

## ⚙️ 4. Execute Instructions

Implement execution logic using pattern matching:

```rust
match instr {
    Instruction::Add { rd, rs1, rs2 } => {
        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
        self.pc += 4;
    }
    // ...
}
```

---

## 🧱 5. Unified Memory Model

**`memory.rs`:** Flat memory array with read/write methods.

```rust
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn read_u32(&self, addr: usize) -> u32 { /* ... */ }
    pub fn write_u32(&mut self, addr: usize, val: u32) { /* ... */ }
}
```

---

## 🚍 6. CPU ↔️ Memory Bus

**`bus.rs`:** Connect CPU and memory.

```rust
pub struct Bus {
    pub memory: Memory,
}

impl Bus {
    pub fn load(&self, addr: u32) -> u32 { /* ... */ }
    pub fn store(&mut self, addr: u32, val: u32) { /* ... */ }
}
```

---

## 📦 7. ELF Loader

Use the `goblin` crate to parse ELF binaries and load segments into memory.

```toml
# Cargo.toml
[dependencies]
goblin = "0.7"
```

---

## 🚨 8. Trap Handling & Syscalls

**`trap.rs`:** Handle `ecall` for basic syscall emulation (later, RPC stub).

```rust
match syscall_id {
    1 => println!("Hello from ecall!"),
    _ => panic!("Unsupported syscall"),
}
```

---

## 🔎 9. Debugging Interface

- Add logging of instructions and register state
- Optional: implement GDB stub (`gdbstub` crate)

---

## ✅ 10. Run Test Programs

- Compile RISC-V ELF binaries with:
  ```bash
  riscv64-unknown-elf-gcc -nostdlib -o test.elf test.c
  ```
- Load into memory and execute step-by-step
- Verify correct behavior

---

## 🧪 Bonus Enhancements

| Feature              | Purpose                           |
|----------------------|-----------------------------------|
| MMIO Devices         | Emulate UART, timers, etc.        |
| CSR Support          | Handle system-level instructions  |
| Ring Buffers         | Foundation for RPC                |
| Syscall Rollback     | Speculative syscall testing       |
| Linux Compatibility  | Translate Linux syscalls to RPC   |

---

## 📁 Suggested Directory Layout

```
riscv_emulator/
├── src/
│   ├── main.rs
│   ├── cpu.rs
│   ├── memory.rs
│   ├── bus.rs
│   ├── elf_loader.rs
│   ├── trap.rs
├── Cargo.toml
```

---