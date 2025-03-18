# RISC-V Project

* Core components of the RISC-V Emulator
  * Basic RISC-V CPU Execution
    * Implement an interpreter for RISC-V instructions (RV32I or RV64I).
    * Use Rust's strong type system to enforce correct instruction decoding.
    * Implement a trap mechanism for handling exceptions/syscalls
  * Unified Memory Management
    * Implement a global memory model instead of per-process virtual memory.
    * Support segmenation or capability-based access control, e.g CHERI-like pointers.
    * Use a global arena allocator for efficient memory handling.
  * I/O & Ring Buffer-Based RPC
    * Simulate Inter-Process Communication (IPC) using ring buffers.
    * Implement message queues & event-drive execution for system calls.
    * Provide an efficient zero-copy mechanism for IPC
  * Microkernel Design
    * Only provide essential OS in the kernel:
      * Thread scheduling
      * Memory allocation
      * IPC via ring buffers
    * Move everything else to userspace services (e.g. file system, networking)
  * Library Services Instead of Shared Libraries
    * Instead of traditional libc.so, provide system-wide library servers.
    * Processes interact with these servers via RPC stubs mapped into their address space.
    * Versioning system: allow processes request a specific library version
    * Linux Compatibility Layer
      * Implement a syscall translation layer for linux application
      * Provide a runtime like WSL or Linuxulator that maps Linux syscalls to native OS services.
  * Roadmap & Development Stages
    * Phase 1: Build the RISC-V emulator in Rust
      * Step 1: create a simple RISC-V instruction interpreter
      * Step 2: implement basic memory emulation (flat model first, then segmented/capability-based memory)
      * Step 3: support loading ELF binaries compiled for the architecture
      * Step 4: implement a minimal debugging interface (GDB stub or logging console).
      * Outcome: a working Rust-based RISC-V emulator that can load and run basic test programs
    * Phase 2: Implement the Microkernel OS
      * Step 5: Implement task scheduling & multitasking in the microkernel
      * Step 6: Implement a global memory allocator instead of per-processing paging.
      * Step 7: Implement a ring buffer-based RPC for process communication.
      * Step 8: Implement basic system calls (file I/O, process control).
      * Outcome: a microkernel that can run multiple tasks and provide system services through RPC
    * Phase 3: Implement Library Services Instead of Shared Libraries
      * Step 9: Design an RPC-based library service model
      * Step 10: Implement a runtime linker that provides function stubs to applications.
      * Step 11: Implement versioning & compatibility modes (e.g. allow processes to request libc v2.1).
      * Step 12: Optimize ring buffer performance for IPC-heavy worloads
      * Outcome: Applications can call library functions through a centralized service instead of linking against shared objects.
    * Phase 4: Add a Linux Compatibility Layer
      * Step 13: Implement a Linux syscall translation layer.
      * Step 14: Implement basic Linux process emulation, e.g. fork(), execve()
      * Step 15: Implement file system mapping between Linux paths and the microkernels storage.
      * Step 16: Implement dynamic linking for compatibility mode (so linux apps use the compatibility layer).
      * Outcome: The OS can run both native application and linux binaries in compatibility mode.
    * Recommended Rust Crates & Tools
      * For RISC-V CPU Emulation
        * riscv - provide risc-v instruction support
        * qemu-rs - rust bindings for QEMU
      * For Memory & Execution Handling
        * memmap2 - memory mapping support
        * region - virtual memory allocation & protection
        * buddy_allocator - buddy system allocator for global memory management.
      * For Linux Compatibility
        * nix - Unix-like system calls in Rust.
        * syscalls - Raw syscall interface.
  