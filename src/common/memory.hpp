#pragma once

#include <types.h>
#include <traits.hpp>

#include <builtins/memory.h>


template <typename T>
class PrivateConstPtr {
public:
    typedef T type;
    typedef const T *ptr_type;
private:
    const T *ptr = nullptr;
public:
    PrivateConstPtr() = default;
    explicit PrivateConstPtr(const T *p) : ptr(p) {}

    bool is_null() const {
        return ptr == nullptr;
    }

    const T *data() const {
        return ptr;
    }
    T load(size_t i) const {
        return ptr[i];
    }
    void load(size_t i, T *v) const {
        *v = load(i);
    }
    void load(size_t i, T &v) const {
        v = load(i);
    }
    void load_many(size_t i, size_t n, T *v) {
        for (size_t j = 0; j < n; ++j) {
            load(i + j, v + j);
        }
    }
    const T &operator[](size_t i) const {
        return ptr[i];
    }

    template <typename S>
    PrivateConstPtr<S> reinterpret() {
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment");
        return PrivateConstPtr<S>(reinterpret_cast<const S *>(ptr));
    }
};

template <typename T>
class PrivatePtr {
public:
    typedef T type;
    typedef T *ptr_type;
private:
    T *ptr = nullptr;
public:
    PrivatePtr() = default;
    explicit PrivatePtr(T *p) : ptr(p) {}

    bool is_null() const {
        return ptr == nullptr;
    }

    T *data() {
        return ptr;
    }
    const T *data() const {
        return ptr;
    }
    void store(size_t i, const T *v) {
        ptr[i] = v;
    }
    void store(size_t i, const T &v) {
        store(i, &v);
    }
    void store_many(size_t i, size_t n, const T *v) {
        for (size_t j = 0; j < n; ++j) {
            store(i + j, v + j);
        }
    }
    T load(size_t i) const {
        return ptr[i];
    }
    void load(size_t i, T *v) const {
        *v = load(i);
    }
    void load(size_t i, T &v) const {
        v = load(i);
    }
    void load_many(size_t i, size_t n, T *v) {
        for (size_t j = 0; j < n; ++j) {
            load(i + j, v + j);
        }
    }
    T &operator[](size_t i) {
        return ptr[i];
    }
    const T &operator[](size_t i) const {
        return ptr[i];
    }

    PrivateConstPtr<T> const_() const {
        return PrivateConstPtr<T>(ptr);
    }
    template <typename S>
    PrivatePtr<S> reinterpret() {
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment");
        return PrivatePtr<S>(reinterpret_cast<S *>(ptr));
    }
};

#define DEFINE_POINTER_BUILTIN_CONST(A, P) \
template <typename T> \
class P##Ptr { \
public: \
    typedef T type; \
    typedef const T *ptr_type; \
private: \
    typedef size_t l_ptr_type; \
    l_ptr_type ptr; \
private: \
    static_assert((sizeof(T) % alignof(T)) == 0, "Size is not multiple of alignment"); \
    static constexpr bool is_aligned(int p) { \
        int i = 3; \
        for (; i > p; --i) { \
            if ((alignof(T) % (1<<i)) == 0) { \
                return false; \
            } \
        } \
        return (alignof(T) % (1<<p)) == 0; \
    } \
    template <typename U=T> \
    enable_if<is_aligned(3), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<3); \
        xm_load_##A##_ulong((size_t)ptr, i*N, N, (ulong *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(2), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<2); \
        xm_load_##A##_uint((size_t)ptr, i*N, N, (uint *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(1), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<1); \
        xm_load_##A##_ushort((size_t)ptr, i*N, N, (ushort *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(0), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<0); \
        xm_load_##A##_uchar((size_t)ptr, i*N, N, (uchar *)v); \
    } \
public: \
    P##Ptr() = default; \
    explicit P##Ptr(l_ptr_type p) : ptr(p) {} \
    bool is_null() const { \
        return xm_is_null_global_uchar(ptr); \
    } \
    void load(size_t i, T *v) const { \
        l_load(i, 1, v); \
    } \
    void load(size_t i, T &v) const { \
        load(i, &v); \
    } \
    T load(size_t i) const { \
        T t; \
        load(i, &t); \
        return t; \
    } \
    void load_many(size_t i, size_t n, T *v) { \
        l_load(i, n, v); \
    } \
    T operator[](size_t i) const { \
        return load(i); \
    } \
    template <typename S> \
    P##Ptr<S> reinterpret() { \
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment"); \
        return P##Ptr<S>(ptr); \
    } \
}; \

#define DEFINE_POINTER_BUILTIN(A, P) \
DEFINE_POINTER_BUILTIN_CONST(A, P##Const) \
template <typename T> \
class P##Ptr { \
public: \
    typedef T type; \
    typedef const T *ptr_type; \
private: \
    typedef size_t l_ptr_type; \
    l_ptr_type ptr; \
private: \
    static_assert((sizeof(T) % alignof(T)) == 0, "Size is not multiple of alignment"); \
    static constexpr bool is_aligned(int p) { \
        int i = 3; \
        for (; i > p; --i) { \
            if ((alignof(T) % (1<<i)) == 0) { \
                return false; \
            } \
        } \
        return (alignof(T) % (1<<p)) == 0; \
    } \
    template <typename U=T> \
    enable_if<is_aligned(3), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<3); \
        xm_load_##A##_ulong((size_t)ptr, i*N, N, (ulong *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(2), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<2); \
        xm_load_##A##_uint((size_t)ptr, i*N, N, (uint *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(1), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<1); \
        xm_load_##A##_ushort((size_t)ptr, i*N, N, (ushort *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(0), void> l_load(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<0); \
        xm_load_##A##_uchar((size_t)ptr, i*N, N, (uchar *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(3), void> l_store(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<3); \
        xm_store_##A##_ulong((size_t)ptr, i*N, N, (const ulong *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(2), void> l_store(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<2); \
        xm_store_##A##_uint((size_t)ptr, i*N, N, (const uint *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(1), void> l_store(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<1); \
        xm_store_##A##_ushort((size_t)ptr, i*N, N, (const ushort *)v); \
    } \
    template <typename U=T> \
    enable_if<is_aligned(0), void> l_store(size_t i, size_t n, T *v) { \
        int N = sizeof(T)/(1<<0); \
        xm_store_##A##_uchar((size_t)ptr, i*N, N, (const uchar *)v); \
    } \
public: \
    P##Ptr() = default; \
    explicit P##Ptr(l_ptr_type p) : ptr(p) {} \
    bool is_null() const { \
        return xm_is_null_global_uchar(ptr); \
    } \
    void load(size_t i, T *v) const { \
        l_load(i, 1, v); \
    } \
    void load(size_t i, T &v) const { \
        load(i, &v); \
    } \
    T load(size_t i) const { \
        T t; \
        load(i, &t); \
        return t; \
    } \
    void load_many(size_t i, size_t n, T *v) { \
        l_load(i, n, v); \
    } \
    void store(size_t i, const T *v) { \
        l_store(i, 1, v); \
    } \
    void store(size_t i, const T &v) { \
        store(i, &v); \
    } \
    void store_many(size_t i, size_t n, const T *v) { \
        l_store(i, n, v); \
    } \
    T operator[](size_t i) const { \
        return load(i); \
    } \
    P##ConstPtr<T> const_() const { \
        return P##ConstPtr<T>(ptr); \
    } \
    template <typename S> \
    P##Ptr<S> reinterpret() { \
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment"); \
        return P##Ptr<S>(ptr); \
    } \
}; \


//DEFINE_POINTER_BUILTIN(private, Private)
DEFINE_POINTER_BUILTIN(local, Local)
DEFINE_POINTER_BUILTIN(global, Global)
DEFINE_POINTER_BUILTIN_CONST(constant, Constant)
