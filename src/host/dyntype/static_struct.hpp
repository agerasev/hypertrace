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
    public:
        std::array<std::string, sizeof...(Types)> field_names_;

    private:
        struct Unzipper {
            rstd::Tuple<std::array<std::string, sizeof...(Types)>, rstd::Tuple<rstd::Box<Types>...>> operator()(
                rstd::Tuple<std::string, rstd::Box<typename Types::Instance>> &&...insts
            ) {
                return rstd::Tuple(
                    std::array{insts.template get<0>()...},
                    rstd::Tuple(std::forward<rstd::Box<Types>>(insts.template get<1>())...)
                );
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
            BaseInstance(std::forward<rstd::Box<Types>>(flds.template get<1>())...),
            field_names_{flds.template get<0>()...}
        {}
        /*
        std::vector<ItemInstanceBox> &fields() { return BaseInstance::fields(); }
        const std::vector<ItemInstanceBox> &fields() const { return BaseInstance::fields(); }

        std::vector<ItemInstanceBox> &field_names() {
            return field_names_;
        }
        const std::vector<ItemInstanceBox> &field_names() const {
            return field_names_;
        }

        Struct type_() const {
            return Struct(
                rstd::iter_ref(field_names_).zip(rstd::iter_ref(fields()))
                .map([](auto &&nf) { return rstd::Tuple<std::string, ItemTypeBox>(rstd::clone(*rstd::get<0>(nf)), (*rstd::get<1>(nf))->type()); })
            );
        }
        virtual Struct *_type() const override {
            return new Struct(type_());
        }
        rstd::Box<Struct> type() const {
            return rstd::Box<Struct>::_from_raw(_type());
        }
        */
    };
    /*
private:
    std::vector<std::string> field_names_;

public:
    Struct() = default;
    template <typename I>
    Struct(I &&field_iter) {
        for (auto &&t : field_iter) {
            append(rstd::get<0>(t), std::move(rstd::get<1>(t)));
        }
    }
    void append(const std::string &str, ItemTypeBox &&i) {
        Base::append(std::move(i));
        field_names_.push_back(str);
    }
    
    std::vector<ItemTypeBox> &fields() { return Base::fields(); }
    const std::vector<ItemTypeBox> &fields() const { return Base::fields(); }

    std::vector<ItemTypeBox> &field_names() {
        return field_names_;
    }
    const std::vector<ItemTypeBox> &field_names() const {
        return field_names_;
    }

    virtual Struct *_clone() const override {
        return new Struct(
            rstd::iter_ref(field_names_).zip(rstd::iter_ref(fields()))
            .map([](auto &&nf) { return rstd::Tuple<std::string, ItemTypeBox>(rstd::clone(*rstd::get<0>(nf)), (*rstd::get<1>(nf))->clone()); })
        );
    }
    rstd::Box<Struct> clone() const {
        return rstd::Box<Struct>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
        hasher._hash_raw(Base::id());
        hasher._hash_raw(typeid(Struct).hash_code());
        for (const std::string &n : field_names_) {
            hasher.hash(n);
        }
        return hasher.finish();
    }

    Instance load_(const uchar *src) const {
        BaseInstance dtup = Base::load_(src);
        Instance dst;
        for (size_t i = 0; i < field_names_.size(); ++i) {
            dst.append(field_names_[i], std::move(dtup.fields()[i]));
        }
        return dst;
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("Struct{}", id());
    }
    virtual Source source() const override {
        return Base::source_with_names([&](size_t i){ return field_names_[i]; });
    }
    */
};

} // namespace dyn
