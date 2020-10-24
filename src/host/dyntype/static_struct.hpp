#pragma once

#include <array>
#include <rstd.hpp>
#include "static_tuple.hpp"


namespace dyn {

template <typename ...Types>
class StaticStruct : public StaticTuple<Types...> {
protected:
    typedef StaticTuple<Types...> Base;
    typedef typename StaticTuple<Types...>::Instance BaseInstance;

public:
    class Instance_ : public BaseInstance {
    private:
        std::array<std::string, sizeof...(Types)> field_names_;

        struct Unzipper {
            rs::Tuple<std::array<std::string, sizeof...(Types)>, rs::Tuple<rs::Box<typename Types::Instance>...>> operator()(
                rs::Tuple<std::string, rs::Box<typename Types::Instance>> &&...insts
            ) {
                return rs::Tuple(
                    std::array{insts.template get<0>()...},
                    rs::Tuple(std::forward<rs::Box<typename Types::Instance>>(insts.template get<1>())...)
                );
            }
        };
        struct TypeExtractor {
            rs::Tuple<rs::Box<Types>...> operator()(const rs::Box<typename Types::Instance> &...insts) {
                return rs::Tuple(insts->type()...);
            }
        };

    public:
        Instance_() = default;
        explicit Instance_(rs::Tuple<rs::Tuple<std::string, rs::Box<typename Types::Instance>>...> &&flds) {
            auto ret = flds.unpack(Unzipper());
            field_names_ = std::move(ret.template get<0>());
            BaseInstance::fields() = std::move(ret.template get<1>());
        }
        explicit Instance_(rs::Tuple<std::string, rs::Box<typename Types::Instance>> &&...flds) :
            BaseInstance(std::forward<rs::Box<typename Types::Instance>>(flds.template get<1>())...),
            field_names_{flds.template get<0>()...}
        {}
        Instance_(const std::array<std::string, sizeof...(Types)> &names, rs::Tuple<rs::Box<typename Types::Instance>...> &&insts) :
            BaseInstance(std::move(insts)),
            field_names_(names)
        {}

        rs::Tuple<rs::Box<typename Types::Instance>...> &fields() { return BaseInstance::fields(); }
        const rs::Tuple<rs::Box<typename Types::Instance>...> &fields() const { return BaseInstance::fields(); }

        std::array<std::string, sizeof...(Types)> &field_names() {
            return field_names_;
        }
        const std::array<std::string, sizeof...(Types)> &field_names() const {
            return field_names_;
        }

        StaticStruct type_() const {
            return StaticStruct(field_names_, fields().unpack_ref(TypeExtractor()));
        }
        virtual StaticStruct *_type() const override {
            return new StaticStruct(type_());
        }
        rs::Box<StaticStruct> type() const {
            return rs::Box<StaticStruct>::_from_raw(_type());
        }
    };
    typedef Instance_ Instance;

private:
    std::array<std::string, sizeof...(Types)> field_names_;

    struct Unzipper {
        rs::Tuple<std::array<std::string, sizeof...(Types)>, rs::Tuple<rs::Box<Types>...>> operator()(
            rs::Tuple<std::string, rs::Box<Types>> &&...insts
        ) {
            return rs::Tuple(
                std::array{insts.template get<0>()...},
                rs::Tuple(std::forward<rs::Box<Types>>(insts.template get<1>())...)
            );
        }
    };

public:
    StaticStruct() = default;
    explicit StaticStruct(rs::Tuple<rs::Tuple<std::string, rs::Box<Types>>...> &&flds) {
        auto ret = flds.unpack(Unzipper());
        field_names_ = std::move(ret.template get<0>());
        Base::fields() = std::move(ret.template get<1>());
    }
    explicit StaticStruct(rs::Tuple<std::string, rs::Box<Types>> &&...flds) :
        Base(std::forward<rs::Box<Types>>(flds.template get<1>())...),
        field_names_{flds.template get<0>()...}
    {}
    StaticStruct(const std::array<std::string, sizeof...(Types)> &names, rs::Tuple<rs::Box<Types>...> &&insts) :
        Base(std::move(insts)),
        field_names_(names)
    {}

    rs::Tuple<rs::Box<Types>...> &fields() { return Base::fields(); }
    const rs::Tuple<rs::Box<Types>...> &fields() const { return Base::fields(); }

    std::array<std::string, sizeof...(Types)> &field_names() {
        return field_names_;
    }
    const std::array<std::string, sizeof...(Types)> &field_names() const {
        return field_names_;
    }

private:
    struct Cloner {
        rs::Tuple<rs::Box<Types>...> operator()(const rs::Box<Types> &...types) {
            return rs::Tuple(types->clone()...);
        }
    };

public:
    virtual StaticStruct *_clone() const override {
        return new StaticStruct(field_names(), fields().unpack_ref(Cloner()));
    }
    rs::Box<StaticStruct> clone() const {
        return rs::Box<StaticStruct>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(Base::id());
        hasher._hash_raw(typeid(StaticStruct).hash_code());
        for (const std::string &n : field_names_) {
            hasher.hash(n);
        }
        return hasher.finish();
    }

    Instance load_(const uchar *src) const {
        BaseInstance dtup = Base::load_(src);
        return Instance(field_names(), std::move(dtup.fields()));
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("StaticStruct{}", id());
    }
    virtual Source source() const override {
        return Base::source_with_names([&](size_t i){ return field_names_[i]; });
    }
};

} // namespace dyn
