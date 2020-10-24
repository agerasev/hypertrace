#pragma once

#include <array>
#include <rstd/prelude.hpp>
#include "static_tuple.hpp"


namespace dyn {

template <typename ...Types>
class StaticStruct : public StaticTuple<Types...> {
protected:
    typedef StaticTuple<Types...> Base;
    typedef typename StaticTuple<Types...>::Instance BaseInstance;

public:
    class Instance : public BaseInstance {
    private:
        std::array<std::string, sizeof...(Types)> field_names_;

        struct Unzipper {
            rstd::Tuple<std::array<std::string, sizeof...(Types)>, rstd::Tuple<rstd::Box<typename Types::Instance>...>> operator()(
                rstd::Tuple<std::string, rstd::Box<typename Types::Instance>> &&...insts
            ) {
                return rstd::Tuple(
                    std::array{insts.template get<0>()...},
                    rstd::Tuple(std::forward<rstd::Box<typename Types::Instance>>(insts.template get<1>())...)
                );
            }
        };
        struct TypeExtractor {
            rstd::Tuple<rstd::Box<Types>...> operator()(const rstd::Box<typename Types::Instance> &...insts) {
                return rstd::Tuple(insts->type()...);
            }
        };

    public:
        Instance() = default;
        explicit Instance(rstd::Tuple<rstd::Tuple<std::string, rstd::Box<typename Types::Instance>>...> &&flds) {
            auto ret = flds.unpack(Unzipper());
            field_names_ = std::move(ret.template get<0>());
            BaseInstance::fields() = std::move(ret.template get<1>());
        }
        explicit Instance(rstd::Tuple<std::string, rstd::Box<typename Types::Instance>> &&...flds) :
            BaseInstance(std::forward<rstd::Box<typename Types::Instance>>(flds.template get<1>())...),
            field_names_{flds.template get<0>()...}
        {}
        Instance(const std::array<std::string, sizeof...(Types)> &names, rstd::Tuple<rstd::Box<typename Types::Instance>...> &&insts) :
            BaseInstance(std::move(insts)),
            field_names_(names)
        {}

        rstd::Tuple<rstd::Box<typename Types::Instance>...> &fields() { return BaseInstance::fields(); }
        const rstd::Tuple<rstd::Box<typename Types::Instance>...> &fields() const { return BaseInstance::fields(); }

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
        rstd::Box<StaticStruct> type() const {
            return rstd::Box<StaticStruct>::_from_raw(_type());
        }
    };

private:
    std::array<std::string, sizeof...(Types)> field_names_;

    struct Unzipper {
        rstd::Tuple<std::array<std::string, sizeof...(Types)>, rstd::Tuple<rstd::Box<Types>...>> operator()(
            rstd::Tuple<std::string, rstd::Box<Types>> &&...insts
        ) {
            return rstd::Tuple(
                std::array{insts.template get<0>()...},
                rstd::Tuple(std::forward<rstd::Box<Types>>(insts.template get<1>())...)
            );
        }
    };

public:
    StaticStruct() = default;
    explicit StaticStruct(rstd::Tuple<rstd::Tuple<std::string, rstd::Box<Types>>...> &&flds) {
        auto ret = flds.unpack(Unzipper());
        field_names_ = std::move(ret.template get<0>());
        Base::fields() = std::move(ret.template get<1>());
    }
    explicit StaticStruct(rstd::Tuple<std::string, rstd::Box<Types>> &&...flds) :
        Base(std::forward<rstd::Box<Types>>(flds.template get<1>())...),
        field_names_{flds.template get<0>()...}
    {}
    StaticStruct(const std::array<std::string, sizeof...(Types)> &names, rstd::Tuple<rstd::Box<Types>...> &&insts) :
        Base(std::move(insts)),
        field_names_(names)
    {}

    rstd::Tuple<rstd::Box<Types>...> &fields() { return Base::fields(); }
    const rstd::Tuple<rstd::Box<Types>...> &fields() const { return Base::fields(); }

    std::array<std::string, sizeof...(Types)> &field_names() {
        return field_names_;
    }
    const std::array<std::string, sizeof...(Types)> &field_names() const {
        return field_names_;
    }

private:
    struct Cloner {
        rstd::Tuple<rstd::Box<Types>...> operator()(const rstd::Box<Types> &...types) {
            return rstd::Tuple(types->clone()...);
        }
    };

public:
    virtual StaticStruct *_clone() const override {
        return new StaticStruct(field_names(), fields().unpack_ref(Cloner()));
    }
    rstd::Box<StaticStruct> clone() const {
        return rstd::Box<StaticStruct>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
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
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("StaticStruct{}", id());
    }
    virtual Source source() const override {
        return Base::source_with_names([&](size_t i){ return field_names_[i]; });
    }
};

} // namespace dyn
