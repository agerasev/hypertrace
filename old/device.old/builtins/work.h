#pragma once

#include <types.h>


#ifdef __cplusplus
extern "C" {
#endif

uint x_get_work_dim();
size_t x_get_global_size(uint);
size_t x_get_global_id(uint);
size_t x_get_local_size(uint);
size_t x_get_local_id(uint);
size_t x_get_num_groups(uint);
size_t x_get_group_id(uint);
size_t x_get_global_offset(uint);

#ifdef __cplusplus
};
#endif
