#pragma once

#include <builtins/work.h>


namespace work {
    inline uint get_work_dim() {
        return x_get_work_dim();
    }
    inline size_t get_global_size(uint x) {
        return x_get_global_size(x);
    }
    inline size_t get_global_id(uint x) {
        return x_get_global_id(x);
    }
    inline size_t get_local_size(uint x) {
        return x_get_local_size(x);
    }
    inline size_t get_local_id(uint x) {
        return x_get_local_id(x);
    }
    inline size_t get_num_groups(uint x) {
        return x_get_num_groups(x);
    }
    inline size_t get_group_id(uint x) {
        return x_get_group_id(x);
    }
    inline size_t get_global_offset(uint x) {
        return x_get_global_offset(x);
    }
};
