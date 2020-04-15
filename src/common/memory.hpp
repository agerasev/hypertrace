#pragma once

#include <types.h>


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


#define DEFINE_POINTER_BUILTIN_CONST(NS, P) \
template <typename T> \
class P##Ptr { \
public: \
    typedef T type; \
    typedef const T *ptr_type; \
private: \
    typedef const void __attribute__((aligned(alignof(T)))) *l_const_ptr_type; \
    typedef void __attribute__((aligned(alignof(T)))) *l_ptr_type; \
    l_const_ptr_type ptr; \
public: \
    P##Ptr() = default; \
    explicit P##Ptr(l_const_ptr_type p) : ptr(p) {} \
    bool is_null() const { \
        return xm_is_null_##NS(ptr); \
    } \
    void load(size_t i, T *v) const { \
        xm_load_##NS(ptr, i*sizeof(T), sizeof(T), (l_ptr_type)v); \
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
        xm_load_##NS(ptr, i*sizeof(T), n*sizeof(T), (l_ptr_type)v); \
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

#define DEFINE_POINTER_BUILTIN(NS, P) \
template <typename T> \
class P##Ptr { \
public: \
    typedef T type; \
    typedef T *ptr_type; \
private: \
    typedef const void __attribute__((aligned(alignof(T)))) *l_const_ptr_type; \
    typedef void __attribute__((aligned(alignof(T)))) *l_ptr_type; \
    l_ptr_type ptr = nullptr; \
public: \
    P##Ptr() = default; \
    explicit P##Ptr(l_ptr_type p) : ptr(p) {} \
    bool is_null() const { \
        return xm_is_null_##NS(ptr); \
    } \
    void load(size_t i, T *v) const { \
        xm_load_##NS(ptr, i*sizeof(T), sizeof(T), (l_ptr_type)v); \
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
        xm_load_##NS(ptr, i*sizeof(T), n*sizeof(T), (l_ptr_type)v); \
    } \
    void store(size_t i, const T *v) { \
        xm_store_##NS(ptr, i*sizeof(T), sizeof(T), (l_const_ptr_type)v); \
    } \
    void store(size_t i, const T &v) { \
        store(i, &v); \
    } \
    void store_many(size_t i, size_t n, const T *v) { \
        xm_store_##NS(ptr, i*sizeof(T), n*sizeof(T), (l_const_ptr_type)v); \
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


//DEFINE_POINTER_BUILTIN_CONST(private, PrivateConst)
//DEFINE_POINTER_BUILTIN(private, Private)
DEFINE_POINTER_BUILTIN_CONST(local, LocalConst)
DEFINE_POINTER_BUILTIN(local, Local)
DEFINE_POINTER_BUILTIN_CONST(global, GlobalConst)
DEFINE_POINTER_BUILTIN(global, Global)
DEFINE_POINTER_BUILTIN_CONST(constant, Constant)
