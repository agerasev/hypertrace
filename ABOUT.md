# About

The library contains so-called **frontend** (host part, which runs on CPU) written in Rust and **backend** (device part, which runs on GPUs and other OpenCL supporting devices) written in OpenCL C for now, but planning to migrate to Rust too (with wider adoption of OpenCL >= 2.1, using some Rust-SPIRV compiler like the one in [rust-gpu](https://github.com/EmbarkStudios/rust-gpu)).

You may add your own functionality or entities like shapes and materials by implementing some interface both in frontend and backend side.
