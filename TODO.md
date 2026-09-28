# TODO

The [constant-curvature migration roadmap](CURVATURE_MIGRATION.md) describes the
shared geometry kernel, spherical support, hyperbolic model migration, and
distance/medium contracts planned across Hypertrace, ccgeom, and vecmat.

## Geometries

- [x] Euclidean geometry
- [x] Lobachevsky (hyperbolic) geometry
- [ ] Spherical geometry

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
+ [ ] Sphere
+ [ ] Equidistant
+ [ ] Triangle

### Spherical

+ [ ] Plane
+ [ ] Sphere
+ [ ] Triangle

## Materials

- [x] Specular material 
- [x] Lambertian material
- [x] Transparent material
- [x] Refraction
- [ ] Diffusion on fog
- [ ] Arbitrary BRDF

## Algorithms

+ [ ] Importance sampling

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
