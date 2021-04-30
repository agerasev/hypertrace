#include <macros.hh>
#include <algebra/real.hh>

#if \
    !defined($Geo) || !defined($geo) || \
    !defined($Map) || !defined($map) || \
    !defined($pos) || !defined($dir)
#error "Some of the macro parameters are not defined."
#endif

#define GeoPos $2($Geo,Pos)
#define GeoDir $2($Geo,Dir)
#define GeoMap $2($Geo,Map)

typedef $pos GeoPos;
typedef $dir GeoDir;
typedef $Map GeoMap;

GeoPos $2($geo,_origin)();
GeoDir $2($geo,_default_dir)();

EuDir $2($geo,_dir_to_local)(GeoPos pos, GeoDir dir);
GeoDir $2($geo,_dir_from_local)(GeoPos pos, EuDir ldir);

real $2($geo,_length)(GeoPos a);
real $2($geo,_distance)(GeoPos a, GeoPos b);

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
GeoDir $2($geo,_dir_at)(GeoPos src_pos, GeoDir src_dir, GeoPos dst_pos);

GeoMap $2($geo,_zshift)(real l);
GeoMap $2($geo,_xshift)(real l);
GeoMap $2($geo,_yshift)(real l);
GeoMap $2($geo,_zrotate)(real phi);
GeoMap $2($geo,_xrotate)(real theta);
GeoMap $2($geo,_yrotate)(real theta);

// Turns direction `dir` to *j*.
GeoMap $2($geo,_look_to)(GeoDir dir);

// Rotatates point `pos` around the origin to make it lay on the z axis.
GeoMap $2($geo,_look_at)(GeoPos pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
GeoMap $2($geo,_move_at)(GeoPos pos);
GeoMap $2($geo,_move_to)(GeoDir dir, real dist);

GeoMap $2($geo,_identity)();
GeoPos $2($geo,_apply_pos)(GeoMap m, GeoPos p);
GeoDir $2($geo,_apply_dir)(GeoMap m, GeoPos p, GeoDir d);
GeoDir $2($geo,_apply_normal)(GeoMap m, GeoPos p, GeoDir d);
GeoMap $2($geo,_chain)(GeoMap a, GeoMap b);
GeoMap $2($geo,_inverse)(GeoMap m);

#undef GeoPos
#undef GeoDir
#undef GeoMap
