# TODO

Current geometry conventions and precision limits are specified in
[GEOMETRY_CONTRACT.md](GEOMETRY_CONTRACT.md). See [DEVELOPMENT.md](DEVELOPMENT.md)
for the validation workflow and outstanding platform checks.

## Geometries

- [x] Euclidean geometry
- [x] Lobachevsky (hyperbolic) geometry
- [x] Spherical geometry

## Shapes

### Euclidean

+ [x] Plane
+ [x] Sphere
+ [x] Cube
+ [ ] Triangle
+ [ ] Distance function
+ [ ] Bezier patch

### Hyperbolic

+ [x] Plane
+ [x] Horosphere
+ [x] Sphere with physical radius
+ [ ] Equidistant
+ [ ] Triangle

### Spherical

+ [x] Geodesic plane (great 2-sphere)
+ [x] Sphere with physical radius
+ [ ] Triangle

## Materials

- [x] Specular material 
- [x] Lambertian material
- [x] Transparent material
- [x] Refraction
- [x] Homogeneous fog with isotropic scattering and RGB albedo
- [ ] Heterogeneous media and anisotropic scattering
- [ ] Arbitrary BRDF

## Algorithms

+ [x] Surface and volume emitter importance sampling with MIS
+ [ ] Environment importance sampling and adaptive light selection
+ [x] Shared constant-curvature ray and isometry kernel
+ [x] Unwrapped hit distances and multi-circuit medium events
+ [x] Camera-relative object preparation in CPU f64
+ [ ] Recenter later path segments for wider hyperbolic numerical range
+ [ ] Validate the shared renderer on additional GPU drivers and in a browser
+ [ ] Benchmark camera-relative uploads for large scenes
+ [ ] Acceleration structures

## Effects

- [ ] Lens blur (depth of field)
- [ ] Motion blur of camera
- [ ] Motion blur of moving objects

## Filters

- [x] Gamma correction
- [ ] Lens flare
- [ ] Noise reduction

## Surface tiling

### Euclidean plane and horosphere

- [x] Square tiling
- [x] Hexagonal tiling

### Hyperbolic plane

- [x] Pentagonal tiling
- [ ] Heptagonal tiling
- [ ] Apeirogonal tiling
