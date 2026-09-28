# Shared geometry implementation contract

This contract fixes the conventions used by the constant-curvature migration.
The kernel uses normalized curvature signs `K = -1, 0, +1` and scalar-first
embedded coordinates `(w, x, y, z)`. Camera forward is local negative z.

**Points, rays, and units**

Curved points satisfy `w*w + K*dot(xyz,xyz) = 1`; hyperbolic points have positive
`w`. Euclidean points have `w=1`. Curved ray tangents satisfy `B(P,V)=0` and
`B(V,V)=K`. Euclidean tangents have `w=0` and unit spatial norm. Tangents describe
unit speed in normalized coordinates. Physical segment distance `s` advances
curved coordinates by `s/R`, where finite positive `R` is the scene curvature
radius. Euclidean `R` is fixed to one. Shapes and medium coefficients use
physical world units, and no map changes the distance reported by a hit.

`advance(P,V,s)` returns both the new point and tangent. Its argument is never
replaced with the shortest distance between endpoints. Spherical phase reduction
is only an implementation detail of coordinate evaluation; the event distance
and accumulated path length retain every complete circuit.

Surface queries use `[minimum, maximum)` physical-distance intervals. New rays
can accept a zero-distance surface event. Repeated-leaf suppression raises the
minimum only for the previously hit leaf, and only by a documented small
physical distance. It never excludes all later intersections with that leaf.
A coplanar ray has no isolated plane crossing; a tangent sphere contact is a
valid isolated root when it falls in the interval.

**Isometries and frames**

An algebra pair `(a,b)` is `a + e*b` with central `e*e=K`. Each component is an
ordinary scalar-first quaternion. Multiplication is
`(a,b)*(c,d)=(a*c+K*b*d, a*d+b*c)`. A unit isometry satisfies
`dot(a,a)+K*dot(b,b)=1` and `dot(a,b)=0`.

Embed a point `(w,xyz)` as the pair `(Quaternion(w,0,0,0), Quaternion(0,xyz))`.
Its image is the corresponding scalar/vector part of
`g * point * combined_conjugate(g)`, where the combined conjugate is
`(conjugate(a), -conjugate(b))`. The same linear action transports tangents.
The inverse unit isometry is `(conjugate(a), conjugate(b))`. Composition remains
`outer.chain(inner)(p) = outer(inner(p))`.

An axis displacement by normalized distance `d` is
`(Quaternion(C_K(d/2),0,0,0), Quaternion(0,axis*S_K(d/2)))`.
A spatial rotation is `(unit_rotation_quaternion, zero)`.

Materials use three-component directions and normals in a local orthonormal
tangent frame. Spherical frames use quaternion multiplication
`V = P * Quaternion(0,local_direction)`, which is defined at antipodes as well.
Hyperbolic frames use the canonical origin-to-point boost; Euclidean frames use
the spatial components. Legacy hyperbolic custom leaves and tilings use explicit
half-space conversions, including direction derivatives.

**CPU and shader boundaries**

The additive CPU API is `ccgeom::embedded::{Space3<T,K>, EmbeddedRay<T>,
EmbeddedIsometry<T,K>, Embedded3<T,K>}` with unit-radius builder aliases
`Flat3`, `Hyperboloid3`, and `Spherical3`. Existing `Euclidean3` and
`Hyperbolic3` keep their coordinate meaning. Legacy maps convert at lowering.

GPU isometries contain the same two quaternion rows, in f32. Keep the original
fixed-record renderer as a comparison fixture during migration. Generated
rendering gets explicit embedded ray/hit types. Existing custom shader leaves
retain their old three-coordinate types through chart adapters; embedded custom
leaves use an explicit schema variant. No source-text rewriting guesses which
coordinate convention a custom shader expects.

Camera-relative preparation composes object transforms with the inverse camera
in CPU f64. Canonical scene data remains unchanged. Directional backgrounds
and object-local material coordinates retain their original frames.

**Events and lighting**

The integrator chooses between the nearest surface and a sampled medium event.
A surface miss gives an unbounded medium interval, including in spherical
space. A medium event after multiple circuits retains its full physical
distance. A vacuum miss evaluates the scene background; the spherical example
uses black initially and obtains light from emissive objects.

The minimal volume deliverable is a homogeneous scalar extinction coefficient
with RGB scattering albedo and isotropic scattering, plus deterministic tests
that force samples beyond several circuits. Free-flight survival and scattering
weights follow the analog estimator; attenuation must not be applied twice.
General heterogeneous media and anisotropic scattering remain follow-up work.
