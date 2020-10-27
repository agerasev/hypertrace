#pragma once

#include <types.h>
#include <traits.hpp>
#include <algebra/vector.hpp>

#ifdef DEVICE
#include <builtins/memory.h>
#endif

enum AddressSpace : uint {
    Private = 0,
    Global = 1,
    Constant = 2,
    Local = 3
};

template <typename T, AddressSpace A=AddressSpace::Private>
struct PtrType {
    typedef __address_space(A) T *type;
    typedef __address_space(A) const T *const_type;
};
template <typename T, AddressSpace A=AddressSpace::Private>
using ptr_type = typename PtrType<T, A>::type;
template <typename T, AddressSpace A=AddressSpace::Private>
using const_ptr_type = typename PtrType<T, A>::const_type;


template <typename T, AddressSpace A>
struct PrivateMemAccess {
    static void load(const_ptr_type<T, A> p, T *v, size_t i, size_t n) {
        for (size_t j = 0; j < n; ++j) {
            v[j] = p[i + j];
        }
    }
    static void store(ptr_type<T, A> p, T *v, size_t i, size_t n) {
        for (size_t j = 0; j < n; ++j) {
            p[i + j] = v[j];
        }
    }
};

#ifdef DEVICE
template <typename T, int MP=3>
static constexpr bool alignment_power() {
    for (int i = MP; i > 0; --i) {
        if ((alignof(T) % (1<<i)) == 0) {
            return i;
        }
    }
    return 0;
}

template <typename T, AddressSpace A, int P>
struct BuiltinMemAccess {};

template <typename T, AddressSpace A>
struct ForeignMemAccess {
    static void load(const_ptr_type<T, A> p, T *v, size_t i, size_t n) {
        BuiltinMemAccess<T, A, alignment_power<T>()>::load(p, v, i, n);
    }
    static void store(ptr_type<T, A> p, const T *v, size_t i, size_t n) {
        BuiltinMemAccess<T, A, alignment_power<T>()>::store(p, v, i, n);
    }
};

#define DEFINE_BUILTIN_MEM_ACCESS_METHOD_LOAD(A, M, P, S) \
    static_assert((sizeof(T) % alignof(T)) == 0, "Size is not multiple of alignment"); \
    static void load(const_ptr_type<T, A> p, T *v, size_t i, size_t n) { \
        static const int N = sizeof(T)/(1<<P); \
        xm_load_##M##_##S((const_ptr_type<S, A>)p, i*N, N, (S *)v); \
    } \

#define DEFINE_BUILTIN_MEM_ACCESS_METHOD_STORE(A, M, P, S) \
    static void store(ptr_type<T, A> p, const T *v, size_t i, size_t n) { \
        static const int N = sizeof(T)/(1<<P); \
        xm_store_##M##_##S((ptr_type<S, A>)p, i*N, N, (const S *)v); \
    } \

#define DEFINE_BUILTIN_MEM_ACCESS_CONST(A, M, P, S) \
template <typename T> \
struct BuiltinMemAccess<T, A, P> { \
DEFINE_BUILTIN_MEM_ACCESS_METHOD_LOAD(A, M, P, S) \
}; \

#define DEFINE_BUILTIN_MEM_ACCESS(A, M, P, S) \
template <typename T> \
struct BuiltinMemAccess<T, A, P> { \
DEFINE_BUILTIN_MEM_ACCESS_METHOD_LOAD(A, M, P, S) \
DEFINE_BUILTIN_MEM_ACCESS_METHOD_STORE(A, M, P, S) \
}; \

#define DEFINE_BUILTIN_MEM_ACCESS_ALL(P, S) \
DEFINE_BUILTIN_MEM_ACCESS(AddressSpace::Local, local, P, S) \
DEFINE_BUILTIN_MEM_ACCESS(AddressSpace::Global, global, P, S) \
DEFINE_BUILTIN_MEM_ACCESS_CONST(AddressSpace::Constant, constant, P, S) \

DEFINE_BUILTIN_MEM_ACCESS_ALL(3, ulong) \
DEFINE_BUILTIN_MEM_ACCESS_ALL(2, uint) \
DEFINE_BUILTIN_MEM_ACCESS_ALL(1, ushort) \
DEFINE_BUILTIN_MEM_ACCESS_ALL(0, uchar) \


#endif

template <typename T, AddressSpace A>
struct MemAccessType {
    typedef PrivateMemAccess<T, A> type;
};

#ifdef DEVICE
template <typename T>
struct MemAccessType<T, AddressSpace::Local> {
    typedef ForeignMemAccess<T, AddressSpace::Local> type;
};
template <typename T>
struct MemAccessType<T, AddressSpace::Global> {
    typedef ForeignMemAccess<T, AddressSpace::Global> type;
};
template <typename T>
struct MemAccessType<T, AddressSpace::Constant> {
    typedef ForeignMemAccess<T, AddressSpace::Constant> type;
};
#endif

template <typename T, AddressSpace A>
using MemAccess = typename MemAccessType<T, A>::type;

template <typename T, AddressSpace A>
class GenericConstPtr {
public:
    typedef T type;
    static const AddressSpace addrspace = A;
private:
    const __address_space(A) T *ptr = nullptr;
public:
    GenericConstPtr() = default;
    GenericConstPtr(const __address_space(A) T *p) : ptr(p) {}
    bool is_null() const {
        return ptr == nullptr;
    }
    const __address_space(A) T *raw() const {
        return ptr;
    }
    operator const __address_space(A) T *() const {
        return ptr;
    }
    T load(size_t i=0) const {
        T x;
        MemAccess<T, A>::load(ptr, &x, i, 1);
        return x;
    }
    template <int N>
    vector<T, N> vload(size_t i=0) const {
        return vector<T, N>::vload(ptr, i);
    }
    T operator[](size_t i) const {
        return load(i);
    }
    template <typename S>
    GenericConstPtr<S, A> reinterpret() {
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment");
        return GenericConstPtr<S, A>(reinterpret_cast<const __address_space(A) S *>(ptr));
    }
};

template <typename T, AddressSpace A>
class GenericPtr {
public:
    typedef T type;
    static const AddressSpace addrspace = A;
private:
    __address_space(A) T *ptr = nullptr;
public:
    GenericPtr() = default;
    GenericPtr(__address_space(A) T *p) : ptr(p) {}
    bool is_null() const {
        return ptr == nullptr;
    }
    const __address_space(A) T *raw() const {
        return ptr;
    }
    __address_space(A) T *raw() {
        return ptr;
    }
    operator const __address_space(A) T *() const {
        return ptr;
    }
    operator __address_space(A) T *() {
        return ptr;
    }
    T load(size_t i=0) const {
        T x;
        MemAccess<T, A>::load(ptr, &x, i, 1);
        return x;
    }
    template <int N>
    vector<T, N> vload(size_t i=0) const {
        return vector<T, N>::vload(ptr, i);
    }
    T operator[](size_t i) const {
        return load(i);
    }
    void store(const T &t, size_t i=0) {
        MemAccess<T, A>::store(ptr, &t, i, 1);
    }
    template <int N>
    void vstore(vector<T, N> v, size_t i=0) const {
        return v.vstore(ptr, i);
    }
    GenericConstPtr<T, A> as_const() const {
        return GenericConstPtr<T, A>(ptr);
    }
    template <typename S>
    GenericPtr<S, A> reinterpret() {
        static_assert((alignof(T) % alignof(S)) == 0, "Pointer conversion breaks alignment");
        return GenericPtr<S, A>(reinterpret_cast<const __address_space(A) S *>(ptr));
    }
};

template <typename T> using private_const_ptr = GenericConstPtr<T, AddressSpace::Private>;
template <typename T> using private_ptr = GenericPtr<T, AddressSpace::Private>;
template <typename T> using local_const_ptr = GenericConstPtr<T, AddressSpace::Local>;
template <typename T> using local_ptr = GenericPtr<T, AddressSpace::Local>;
template <typename T> using global_const_ptr = GenericConstPtr<T, AddressSpace::Global>;
template <typename T> using global_ptr = GenericPtr<T, AddressSpace::Global>;
template <typename T> using constant_ptr = GenericConstPtr<T, AddressSpace::Constant>;
