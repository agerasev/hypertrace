#include "work.h"


uint x_get_work_dim() {
    return get_work_dim();
}
size_t x_get_global_size(uint x) {
    return get_global_size(x);
}
size_t x_get_global_id(uint x) {
    return get_global_id(x);
}
size_t x_get_local_size(uint x) {
    return get_local_size(x);
}
size_t x_get_local_id(uint x) {
    return get_local_id(x);
}
size_t x_get_num_groups(uint x) {
    return get_num_groups(x);
}
size_t x_get_group_id(uint x) {
    return get_group_id(x);
}
size_t x_get_global_offset(uint x) {
    return get_global_offset(x);
}
