# HyperTrace

Lobachevsky (hyperbolic) space ray tracer powered by OpenCL.

## Requirements

+ OpenCL
+ SDL2
+ SDL2_Image

## How to use

To build and run example:

```bash
./script/run.sh main
```

You may use name of another example instead of `main` as well.

To select the OpenCL platform and device:

```bash
./script/run.sh <example-name> [platform-no] [device-no]
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

- [x] Square horosphere tiling
- [x] Hexagonal horosphere tiling
- [x] Pentagonal hyperbolic plane tiling
- [ ] Heptagonal hyperbolic plane tiling

### Effects

- [x] Lens blur (depth of field)
- [x] Motion blur
- [x] Motion blur on moving objects

### Post-processing

- [x] Gamma correction
- [ ] Noise reduction (maybe using DL)

### Other improvements

- [ ] Same RNG seed for work group to reduce divergence
