# HyperTrace

Physically based ray tracer powered by OpenCL.

The whole program including **host** part (which runs on CPU) as well as **device** part (which runs on GPUs and other OpenCL supporting devices) is written mostly in **C++**. As much common code as possible is shared between these parts. The code is written in highly modular and generic manner.

The ray tracer supports Euclidean as well as non-Euclidean spaces like Lobachevsky (hyperbolic) space. You may add your own functionality or entities like shapes and materials by writing plain C++ code which could be able to run both on CPU and GPU.

This project is done mostly for educational purpose and intended to use for experimenting with computer graphics.

## Requirements

+ OpenCL 1.2
+ SDL2 with SDL2_Image
+ Clang and LLVM
+ [LLVM-OpenCL](https://github.com/agerasev/llvm-opencl)**\***

**\*** This project uses custom LLVM-based tool to translate LLVM-IR code (to which C++ is compiled) to OpenCL source code. It depends on LLVM code tree so you need to build LLVM and Clang from source first. LLVM project is huge so make sure you have enough space and computational power. You can find the detailed description by the link above.

## Setup

+ Install GCC or Clang.
+ Install Make and CMake.
+ Install `*-opencl-dev` and `*-opencl-icd` for your device.
+ Clone and build LLVM, Clang and [LLVM-OpenCL](https://github.com/agerasev/llvm-opencl).

### Windows

To build on Windows it is recomended to use [MSYS2](https://www.msys2.org/).

To install packages with MSYS2:

```bash
pacman -S \
    gcc \
    make \
    cmake \
    git \
    mingw64/mingw-w64-x86_64-opencl-headers \
    mingw64/mingw-w64-x86_64-opencl-icd-git \
    mingw-w64-x86_64-SDL2 \
    mingw-w64-x86_64-SDL2_image
```

## How to use

To build and run example:

```bash
./script/run.sh hyperbolic main
```

You may use a name of another example instead of `main` as well.

To select the OpenCL platform and device:

```bash
./script/run.sh <example-group> <example-name> [platform-no] [device-no]
```

To list all available platforms and devices enter `-1` as `[platform-no]`.

## Control

In some examples you may fly around the scene using your keyboard and mouse.

+ Use your mouse to look around.
+ `W`, `A`, `S`, `D` - move forward, left, backward and right respectively.
+ `Space`, `C` - move up and down.
+ `Q`, `E` - tilt counter- and clockwise.
+ `Tab` - release/grab mouse pointer.
+ `Esc` - exit application.

## TODO

### Basics

- [ ] Euclidean geometry
- [x] Lobachevsky (hyperbolic) geometry
- [ ] Spherical geometry

### Materials

- [x] Specular material 
- [x] Lambertian material
- [x] Transparent material
- [ ] Refraction
- [ ] Diffusion on fog
- [ ] Arbitrary BRDF

### Surface tiling

#### Euclidean plane and horosphere

- [x] Square tiling
- [x] Hexagonal tiling

#### Hyperbolic plane

- [x] Pentagonal tiling
- [ ] Heptagonal tiling
- [ ] Apeirogonal tiling

### Effects

- [ ] Lens blur (depth of field)
- [ ] Motion blur
- [ ] Motion blur on moving objects

### Post-processing

- [ ] Gamma correction
- [ ] Noise reduction (maybe using DL)

### Other improvements

- [ ] Same RNG seed for work group to reduce divergence
