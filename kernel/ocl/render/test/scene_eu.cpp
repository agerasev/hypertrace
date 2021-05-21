#include <object/test/covered/eu_def.hpp>
#include <view/point/eu.hh>
#include <background/gradient.hh>

struct SceneTestEu {
    PointViewEu view;
    GradBg background;
};

__global const CoveredTestEu *scene_test_eu__object__gc(__global const SceneTestEu *self) {
    return NULL;
}

__global const PointViewEu *scene_test_eu__view__gc(__global const SceneTestEu *self) {
    return &self->view;
}

__global const GradBg *scene_test_eu__background__gc(__global const SceneTestEu *self) {
    return &self->background;
}

#define $Self SceneTestEu
#define $self scene_test_eu
#define $Geo Eu
#define $geo eu
#define $Object CoveredTestEu
#define $object covered_test_eu
#define $ObjectCache CoveredTestEuCache
#define $object_cache covered_test_eu_cache
#define $View PointViewEu
#define $view point_view_eu
#define $Background GradBg
#define $background grad_bg
#define $light_hops 2
#include <render/scene.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Object
#undef $object
#undef $ObjectCache
#undef $object_cache
#undef $View
#undef $view
#undef $Background
#undef $background
#undef $light_hops
