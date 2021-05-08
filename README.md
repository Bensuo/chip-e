# CHIP-E
## Overview
[![Rust](https://github.com/Bensuo/chip-e/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Bensuo/chip-e/actions/workflows/rust.yml)

A very basic emulator for the CHIP-8 instruction set written in Rust, as a way to learn about both emulation and programming in Rust. Loosely based on [this tutorial from Multigesture](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

Currently using SDL2 for graphics and input.

## Current Status

- Most opcodes have been implemented.
- No working input yet.
- No sound (`beep` is printed to the console).

## TODO

- Input handling so games can be played.
- Sound
- Optimising opcode handling (no giant `switch`)
- Hires mode
- Possibly support SCHIP / MegaChip8
