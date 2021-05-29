# Hypertrace

Physically-based non-Euclidean ray tracer powered by OpenCL.

*This project is done mostly for educational purpose.*

## Supported geometries

+ [x] Euclidean geometry.
+ [x] Lobachevsky (or hyperbolic) geometry.
+ [ ] Spherical geometry (planning).

## Requirements

+ Rustc and Cargo
+ OpenCL 1.2 support
+ SDL2 (used by interactive viewer)

## How to use

To build and run example:

```bash
cd main
cargo run --example hy
```

You may use a name of another example instead of `hy` as well.

To select specific OpenCL platform and device:

```bash
cargo run --example <example_name> -- <platform_no> <device_no>
```

To list all available platforms and devices:

```bash
cargo run --example <example_name> -- --list
```

## Control

In some examples you may fly around the scene using your keyboard and mouse.

+ Use your mouse to look around.
+ `W`, `A`, `S`, `D` - move forward, left, backward and right respectively.
+ `Space`, `C` - move up and down.
+ `Q`, `E` - tilt counter- and clockwise.
+ `Tab` - release/grab mouse pointer.
+ `Esc` - exit application.
