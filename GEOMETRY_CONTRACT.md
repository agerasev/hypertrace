# Shared geometry implementation contract

This contract fixes the shared CPU and WGSL geometry conventions.
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
minimum only for the previously hit leaf, by `8*EPS*R` physical distance
(`EPS=1e-6`, with `R=1` for Euclidean space). It never excludes all later
intersections with that leaf.
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
the spatial components. Hyperbolic tilings use explicit half-space coordinates
for classification,
while their transformations use the same quaternion-pair isometries as tracing.
The horosphere normal is computed directly as an ambient unit tangent.

**CPU and shader boundaries**

The CPU API is `ccgeom::embedded::{Space3<T,K>, EmbeddedRay<T>,
EmbeddedIsometry<T,K>, Embedded3<T,K>}` with unit-radius builder aliases
`Flat3`, `Hyperboloid3`, and `Spherical3`. These are curvature aliases for the
same representation. `scene::Geometry` supports these three canonical geometries;
component trait implementations express which geometries they support.

`scene::Transform<G>` stores a canonical isometry of geometry `G`.
`from_isometry` preserves that geometry through `Camera<G>`, `SceneDefinition<G>`,
`CompiledScene<G>`, and `Renderer<G>`. Curvature sign is a type-level constant;
radius is a checked physical value. Mixed-geometry construction fails to compile.
Local motion takes physical distances, angles, and radius explicitly;
translations in x/y/z order precede rotations in x/y/z order.

GPU isometries contain the same two quaternion rows, in f32. Every scene uses
the shared embedded tracing kernel.

Every built-in and downstream shader module uses the embedded `GeoRay`,
`GeoHit`, `GeoTaggedHit`, and `GeoMaterialContext` contracts. Hit distances are
physical. Component-owned WGSL declares module dependencies explicitly, and
component-owned CPU callbacks validate physical radius and encoded words.
No source rewriting guesses a shader's coordinate convention. `GeoHit.valid` is
0 for a miss, 1 for a hit, and 2 for numerical failure; wrappers propagate failure
instead of treating it as an environmental miss. The
[renderer guide](renderer/README.md#component-owned-shader-modules) specifies
entry-point signatures, namespaces and parameter encoding.

Camera-relative preparation composes object transforms with the inverse camera
in CPU f64. Canonical scene data remains unchanged. Directional backgrounds
and object-local material coordinates retain their original frames.

**Events and lighting**

The integrator chooses between the nearest surface and a sampled medium event.
A surface miss gives an unbounded medium interval, including in spherical
space. A medium event after multiple circuits retains its full physical
distance. A vacuum miss evaluates the scene background; the spherical example
uses black initially and obtains light from emissive objects.

`Medium { extinction, albedo }` implements a scalar extinction
coefficient per physical world unit, RGB scattering albedo, and isotropic
scattering. Zero extinction is vacuum; zero albedo is pure absorption. Both
surface and volume events consume the finite bounce budget. Deterministic tests
force samples beyond several circuits. Free-flight survival and scattering
weights follow the analog estimator; attenuation is not applied twice.
The standalone [Euclidean fog scene](examples/src/bin/eu-fog/scene.rs) owns its
adjustable extinction and scattering albedo; the separate
[spherical studio](examples/src/bin/sp/scene.rs) is vacuum. Each owns its
`scene::<H>()` constructor and numerical settings. General heterogeneous media
and anisotropic scattering remain follow-up work.

**Finite precision and numerical termination**

`GeoPath` stores accumulated travel as a high multiple of 1024 physical world
units and a separate remainder in `[0,1024)`. This retains small later segments
without relying on compensated summation that relaxed shader arithmetic can
reassociate away. Individual block increments are exact below `2^34` total
world units; `geo_path_distance` returns a rounded f32 sum for reporting. The
two parts do not replace per-event distances or optical-depth calculations.
Every segment remains f32, and spherical phase reduction cannot recover low
bits already lost in a very large segment. The accounting limit does not
certify coordinate precision over that range.

Camera-relative preparation occurs in CPU f64 before checked f32 upload.
Hyperbolic cancellation can still corrupt distant points, tangents or later
segments. The tested CPU-to-f32 hyperbolic envelope includes distances through
`3R` with `3e-5` invariant tolerance; this is test coverage, not a hard path
cutoff or a global guarantee. Runtime guards reject nonfinite values, the wrong
hyperboloid sheet, and manifold/unit/orthogonality residuals above `1e-2`.
Advancement rejects invalid physical distances/radii, normalized-phase overflow,
and hyperbolic normalized steps above 40 as an emergency exponential bound.
The bound 40 is not a precision guarantee.

Curved sphere radii also require a representable f32 section equation. The
compiler rejects `cos(r/R)` collapsing to either spherical endpoint,
`cosh(r/R)` collapsing to one, nonnormal or nonfinite sine terms, and hyperbolic
cosine-square overflow. The section discriminant treats
`|D| <= 8*f32::EPSILON*(a*a+b*b+c*c)` as an unresolved double root: this is a
backward-error band for a few ULPs of coefficient/arithmetic error, rather than
a distance tolerance. Negative discriminants outside the band remain misses.
Radii must satisfy `S(r/R)^2 > 8*f32::EPSILON*(1+C(r/R)^2)` so that a sphere
viewed from its center is distinct from this degenerate section. For example,
angular radius `0.002` passes while `0.001` and `1e-5` do not; radii too close
to `pi*R` are also rejected. This applies to unit-sphere conveniences and nested
shapes as well as explicit radii. These input checks do not certify grazing
contacts or distant frames at the same accuracy as well-conditioned hits.

A numerical failure terminates the path, retaining accumulated emission and
adding no background. This is an explicit source of truncation bias, alongside
the finite bounce limit. Distances are not clamped to a shorter path. Recentring
later path segments and extending the supported numerical range remain future
work. A medium event after many spherical circuits remains valid when its
coordinate evaluation and physical distance pass these checks.
