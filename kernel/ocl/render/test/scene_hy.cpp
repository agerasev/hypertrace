#include <object/test/covered/hy_def.hpp>
#include <view/point/hy.hh>
#include <background/test/constant/hy_def.hpp>

struct SceneTestHy {
    PointViewHy view;
    ConstBgHy background;
    CoveredTestHy object;
};

#define $Self SceneTestHy
#define $self scene_test_hy
#define $Geo Hy
#define $geo hy
#define $Object CoveredTestHy
#define $object covered_test_hy
#define $View PointViewHy
#define $view point_view_hy
#define $Background ConstBgHy
#define $background const_bg_hy
#include <render/scene.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Object
#undef $object
#undef $View
#undef $view
#undef $Background
#undef $background
