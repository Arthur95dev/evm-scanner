# 🛡️ EVM Bytecode Security Scanner

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A **static analyzer for Ethereum Virtual Machine (EVM) bytecode**, written in Rust.  
It helps detect potential vulnerabilities in smart contracts by scanning raw bytecode for dangerous opcodes and suspicious patterns.

Built as a hands-on tool for learning Web3 security and preparing for real audits.  
Designed to work seamlessly with [Foundry](https://getfoundry.sh/) artifacts.

## 🔍 Features

- **Parses Foundry artifacts** (JSON from `out/`)
- **Extracts deployed bytecode** (`deployedBytecode.object`)
- **Decodes hex into bytes** and correctly traverses EVM instructions
- **Flags dangerous opcodes**:
  - `CALL` – possible reentrancy vector
  - `DELEGATECALL` – code injection risk
  - `CALLCODE` – deprecated but still dangerous
  - `SELFDESTRUCT` – contract destruction
- **Handles PUSH instructions** (PUSH1–PUSH32) to keep byte offsets accurate
- **Reports** offset and opcode name for each finding

> Planned: context-aware reentrancy detection (opcode chains), batch scanning of directories, JSON/HTML report export.

## 🧱 Architecture

```text
src/
├── main.rs         # entry point, file reading, scan loop
└── opcodes.rs      # Opcode enum, Instruction enum, parsing logic
