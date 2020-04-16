#include <iostream>
#include <fstream>
#include <cassert>

#include <types.h>
#include <traits.hpp>

#include <memory.hpp>

//#include "../device/main.h"


template <typename ...Args>
struct Unwinder {};
template <typename T, typename ...Args>
struct Unwinder<T, Args...> {
    template <int P, template <int, typename> typename X, typename ...XArgs>
    static void for_each(XArgs &&...xargs) {
        X<P, T>::call(xargs...);
        Unwinder<Args...>::template for_each<P + 1, X>(
            std::forward<XArgs>(xargs)...
        );
    }
};
template <>
struct Unwinder<> {
    template <int P, template <int, typename> typename X, typename ...XArgs>
    static void for_each(XArgs &&...) {}
};


template <typename T>
struct TypePrinter {
    std::string name;
    template <typename U=T>
    enable_if<!is_prim<U>(), void> print(std::ostream &out) const {
        out << name;
    }
    template <typename U=T>
    enable_if<is_prim<U>(), void> print(std::ostream &out) const {
        out << type_name<T>();
    }
};
template <typename T, AddressSpace A>
struct TypePrinter<GenericPtr<T, A>> {
    std::string name;
    void print(std::ostream &out) const {
        switch (A) {
        case AddressSpace::Global:
            out << "__global ";
            break;
        case AddressSpace::Local:
            out << "__local ";
            break;
        case AddressSpace::Constant:
            out << "__constant ";
            break;
        default:
            break;
        }
        TypePrinter<T>{name}.print(out);
        out << "*";
    }
};
template <typename T, AddressSpace A>
struct TypePrinter<GenericConstPtr<T, A>> {
    std::string name;
    void print(std::ostream &out) const {
        out << "const ";
        TypePrinter<GenericPtr<T, A>>{name}.print(out);
    }
};
template <>
struct TypePrinter<void> {
    std::string name;
    void print(std::ostream &out) const {
        out << "void";
    }
};

template <typename T>
struct StructPrinter {
    std::string name;
    template <typename U=T>
    enable_if<!is_prim<U>(), void> print(std::ostream &out) const {
        out << "typedef struct "
            << "__attribute__((aligned(" << alignof(device_type<T>) << "))) {\n"
            << "\tuchar data[" << sizeof(device_type<T>) << "];\n"
            << "} " << name << ";\n"
            << "\n";
    }
    template <typename U=T>
    enable_if<is_prim<U>(), void> print(std::ostream &out) const {
        // skip primitive type
    }
};
template <typename T, AddressSpace A>
struct StructPrinter<GenericPtr<T, A>> {
    std::string name;
    void print(std::ostream &out) const {
        StructPrinter<T>{name}.print(out);
    }
};
template <typename T, AddressSpace A>
struct StructPrinter<GenericConstPtr<T, A>> {
    std::string name;
    void print(std::ostream &out) const {
        StructPrinter<T>{name}.print(out);
    }
};
template <>
struct StructPrinter<void> {
    std::string name;
    void print(std::ostream &out) const {
        // skip void
    }
};

template <typename F>
class KernelPrinter {};
template <typename R, typename ...Args>
class KernelPrinter<R(Args...)> {
private:
    template <int P, typename T>
    struct StructPrinterWrapper {
        static void call(const std::string &name, std::ostream &out) {
            StructPrinter<T>{name + std::to_string(P)}.print(out);
        }
    };
    template <int P, typename T>
    struct ArgsDefPrinterWrapper {
        static void call(const std::string &name, std::ostream &out) {
            out << "\t";
            TypePrinter<T>{name + std::to_string(P)}.print(out);
            out << " arg_" << P;
            if (P + 1 < sizeof...(Args)) {
                out << ", ";
            }
            out << "\n";
        }
    };

private:
    std::string name;

public:
    KernelPrinter(
        const std::string &name
    ) : name(name)
    {}
    std::string gen_name() const {
        return "gen_kernel_" + name;
    }
    void print_declaration(std::ostream &out) const {
        StructPrinter<R>{name + "Ret"}.print(out);
        Unwinder<Args...>::template for_each<0, StructPrinterWrapper>(
            name + "Arg", out
        );
        TypePrinter<R>{name + "Ret"}.print(out);
        out << " " << name << "(\n";
        Unwinder<Args...>::template for_each<0, ArgsDefPrinterWrapper>(
            name + "Arg", out
        );
        out << ");\n";
        out << "\n";
    }
    void print_definition(std::ostream &out) const {
        out << "__kernel ";
        TypePrinter<R>{name + "Ret"}.print(out);
        out << " " << gen_name() << "(\n";
        Unwinder<Args...>::template for_each<0, ArgsDefPrinterWrapper>(
            name + "Arg", out
        );
        out << ") {\n";
        out << "\t" << name << "(";
        for (int i = 0; i < sizeof...(Args); ++i) {
            out << "arg_" << i;
            if (i + 1 < sizeof...(Args)) {
                out << ", ";
            }
        }
        out << ");\n";
        out << "}\n";
    }
};

template <typename F>
void print_kernel(
    F*,
    std::ostream &out,
    const std::string &name
) {
    KernelPrinter<F> kp(name);
    kp.print_declaration(out);
    kp.print_definition(out);
}


int main(int argc, char *argv[]) {
    assert(argc == 1 + 2);
    
    int path_max_depth = 6;
    int path_max_diffuse_depth = 2;

    // congif.h
    std::ofstream cfg(argv[1]);
    assert(cfg.good());
    cfg << "#pragma once\n\n"
        << "#define PATH_MAX_DEPTH " << path_max_depth << "\n"
        << "#define PATH_MAX_DIFFUSE_DEPTH " << path_max_diffuse_depth << "\n"
        << "\n";
    cfg.close();

    // kernel.cl
    std::ofstream ker(argv[2]);
    assert(ker.good());
    //print_kernel(trace, ker, "trace");
    ker.close();

    return 0;
}
