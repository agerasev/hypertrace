#include <object/test/covered/hy_def.hpp>
#include <view/point/hy.hh>
#include <background/test/constant/hy_def.hpp>

struct SceneTestHy {
    PointViewHy view;
    ConstBg background;
};

__global const CoveredTestHy *scene_test_hy__object__gc(__global const SceneTestHy *self) {
    return NULL;
}

__global const PointViewHy *scene_test_hy__view__gc(__global const SceneTestHy *self) {
    return &self->view;
}

__global const ConstBg *scene_test_hy__background__gc(__global const SceneTestHy *self) {
    return &self->background;
}

#define $Self SceneTestHy
#define $self scene_test_hy
#define $Geo Hy
#define $geo hy
#define $Object CoveredTestHy
#define $object covered_test_hy
#define $View PointViewHy
#define $view point_view_hy
#define $Background ConstBg
#define $background const_bg_hy
#define $light_hops 2
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
#undef $light_hops
