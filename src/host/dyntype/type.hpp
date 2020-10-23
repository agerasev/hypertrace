#pragma once

#include <cstdint>
#include <map>
#include <typeinfo>
#include <rstd/prelude.hpp>
#include <common/types.hh>


namespace dyn {

struct Source {
private:
    std::string name_;
    std::map<std::string, std::string> files_;
public:
    Source() = default;
    inline explicit Source(
        const std::string &n,
        std::map<std::string, std::string> &&f={}
    ) : name_(n), files_(std::move(f)) {}

    inline std::string name() const { return name_; }
    inline const std::map<std::string, std::string> &files() const { return files_; }
    inline std::map<std::string, std::string> &files() { return files_; }
    inline std::map<std::string, std::string> into_files() { return std::move(files_); }

    inline void set_name(const std::string &n) { name_ = n; }
    inline rstd::Result<rstd::Tuple<>, std::string> insert(
        const std::string &fname, std::string &&content
    ) {
        auto it = files_.find(fname);
        if (it == files_.end()) {
            assert_(files_.insert(std::make_pair(fname, std::move(content))).second);
            return rstd::Ok();
        } else {
            if (it->second == content) {
                return rstd::Ok();
            } else {
                return rstd::Err(format_("Same file '{}' but different content", fname));
            }
        }
    }
    inline rstd::Result<rstd::Tuple<>, std::string> append(
        std::map<std::string, std::string> &&other
    ) {
        files_.merge(other);
        for (const auto &s : other) {
            auto di = files_.find(s.first);
            if (di->second != s.second) {
                return rstd::Err(format_("Same file '{}' but different content", s.first));
            }
        }
        return rstd::Ok();
    }
};

class Type {
public:
    class Instance {
    public:
        Instance() = default;
        virtual ~Instance() = default;

        virtual Type *_type() const = 0;
        rstd::Box<Type> type() const { return rstd::Box<Type>::_from_raw(_type()); }

        // Dynamic size of instance. If instance is statically sized, then returns Type::size().
        virtual size_t size() const { return type()->size().unwrap(); };
        virtual size_t align() const { return type()->align(); };

        // Stores the instance to the device. The `dst` pointer should be properly aligned.
        virtual void store(uchar *dst) const = 0;
        virtual void load(const uchar *src) = 0;
    };

    Type() = default;
    virtual ~Type() = default;

    virtual Type *_clone() const = 0;
    rstd::Box<Type> clone() const { return rstd::Box<Type>::_from_raw(_clone()); }

    virtual size_t id() const = 0;

    // Static size of instance.
    // If the size is dynamic then the method returns None.
    virtual rstd::Option<size_t> size() const = 0;
    virtual size_t align() const = 0;

    // Loads instance from device. The `src` pointer should be properly aligned.
    virtual Instance *_load(const uchar *src) const = 0;
    rstd::Box<Instance> load(const uchar *src) const { return rstd::Box<Instance>::_from_raw(_load(src)); }

    virtual std::string name() const = 0;
    virtual Source source() const = 0;
};

template <typename Self, typename Base>
class ImplEmptyType : public Base {
    class Instance final : public Base::Instance {
    public:
        virtual Base *_type() const override { return new Self(); }
        rstd::Box<Base> type() const { return rstd::Box<Base>::_from_raw(_type()); }

        virtual void store(uchar *) const override {}
        virtual void load(const uchar *) override {}
    };

    virtual Base *_clone() const override { return new Self(*static_cast<const Self *>(this)); }
    rstd::Box<Base> clone() const { return rstd::Box<Base>::_from_raw(_clone()); }

    virtual size_t id() const override { return typeid(Self).hash_code(); }

    virtual rstd::Option<size_t> size() const override { return rstd::Some<size_t>(0); }
    virtual size_t align() const override { return 0; }

    virtual Instance *_load(const uchar *) const override { return new Instance(); }
    rstd::Box<Instance> load(const uchar *) const { return rstd::Box(_load()); }

    virtual std::string name() const override { return "void"; }
};

typedef rstd::Box<Type> TypeBox;
typedef rstd::Box<Type::Instance> InstanceBox;

} // namespace dyn
