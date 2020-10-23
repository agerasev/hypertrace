#pragma once

#include <rstd/prelude.hpp>
#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename ...Types>
class StaticTuple : public Type {
public:
    class Instance : public Type::Instance {
    public:
        rstd::Tuple<rstd::Box<typename Types::Instance>...> fields_;

        Instance() = default;
        explicit Instance(rstd::Box<typename Types::Instance> &&...flds) :
            fields_(std::forward<rstd::Box<typename Types::Instance>>(flds)...)
        {}
        explicit Instance(rstd::Tuple<rstd::Box<typename Types::Instance>...> flds) :
            fields_(std::move(flds))
        {}

        rstd::Tuple<rstd::Box<typename Types::Instance>...> &fields() {
            return fields_;
        }
        const rstd::Tuple<rstd::Box<typename Types::Instance>...> &fields() const {
            return fields_;
        }

    private:
        struct TypeExtractor {
            rstd::Tuple<rstd::Box<Types>...> operator()(const rstd::Box<typename Types::Instance> &...insts) {
                return rstd::Tuple(insts->type()...);
            }
        };
    public:
        StaticTuple type_() const {
            return StaticTuple(fields_.unpack_ref(TypeExtractor()));
        }
        virtual StaticTuple *_type() const override {
            return new StaticTuple(type_());
        }
        rstd::Box<StaticTuple> type() const {
            return rstd::Box<StaticTuple>::_from_raw(_type());
        }
        /*
        virtual void store(uchar *dst) const override {
            std::vector<size_t> offt = type_().offsets();
            for (size_t i = 0; i < fields_.size(); ++i) {
                fields_[i]->store(dst + offt[i]);
            }
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
        */
    };

private:
    rstd::Tuple<rstd::Box<Types>...> fields_;

public:
    StaticTuple() = default;
    explicit StaticTuple(rstd::Box<Types> &&...flds) :
        fields_(std::forward<rstd::Box<Types>>(flds)...)
    {}
    explicit StaticTuple(rstd::Tuple<rstd::Box<Types>...> flds) :
        fields_(std::move(flds))
    {}

    rstd::Tuple<rstd::Box<Types>...> &fields() {
        return fields_;
    }
    const rstd::Tuple<rstd::Box<Types>...> &fields() const {
        return fields_;
    }

private:
    struct Cloner {
        rstd::Tuple<rstd::Box<Types>...> operator()(const rstd::Box<Types> &...ts) {
            return rstd::Tuple(ts->clone()...);
        }
    };

    struct Hasher {
        rstd::DefaultHasher *hasher;
        template <size_t P>
        void operator()(const rstd::Box<rstd::nth_type<P, Types...>> &t) {
            hasher->_hash_raw(t->id());
        }
    };

public:
    virtual StaticTuple *_clone() const override {
        return new StaticTuple(fields_.unpack_ref(Cloner()));
    }
    rstd::Box<StaticTuple> clone() const {
        return rstd::Box<StaticTuple>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
        hasher._hash_raw(typeid(StaticTuple).hash_code());
        fields_.visit_ref(Hasher{&hasher});
        return hasher.finish();
    }

private:
    struct OffsetCounter {
        std::vector<size_t> *ofv;
        size_t p = 0;
        template <size_t P>
        void operator()(const rstd::Box<rstd::nth_type<P, Types...>> &t) {
            size_t of = upper_multiple(t->align(), p);
            p = of + t->size().unwrap();
            ofv->push_back(of);
        }
    };
    struct AlignCounter {
        size_t *p;
        template <size_t P>
        void operator()(const rstd::Box<rstd::nth_type<P, Types...>> &t) {
            *p = std::max(t->align(), *p);
        }
    };
    struct SizeCounter {
        size_t *p;
        template <size_t P>
        void operator()(const rstd::Box<rstd::nth_type<P, Types...>> &t) {
            *p = upper_multiple(t->align(), *p) + t->size().unwrap();
        }
    };

public:
    std::vector<size_t> offsets() const {
        std::vector<size_t> ofv;
        fields_.visit_ref(OffsetCounter{&ofv});
        return ofv;
    }
    size_t align() const {
        size_t p = 0;
        fields_.visit_ref(AlignCounter{&p});
        return p;
    }
    rstd::Option<size_t> size() const {
        size_t p = 0;
        fields_.visit_ref(SizeCounter{&p});
        return rstd::Some(std::max(upper_multiple(align(), p), 1));
    }

private:
    struct Loader {
        rstd::Tuple<rstd::Box<typename Types::Instance>...> *ts;
        const uchar *src;
        std::vector<size_t> ofv;
        template <size_t P>
        void operator()(const rstd::Box<rstd::nth_type<P, Types...>> &t) {
            ts->template get<P>() = t->load(src + ofv[P]);
        }
    };

public:
    Instance load_(const uchar *src) const {
        rstd::Tuple<rstd::Box<typename Types::Instance>...> insts;
        fields_.visit_ref(Loader{&insts, src, offsets()});
        return Instance(std::move(insts));
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }

    /*
    virtual std::string name() const override {
        return format_("Tuple{}", id());
    }
    template <typename F>
    Source source_with_names(F field_name) const {
        std::stringstream ss;
        Source src;

        for (const TypeBox &f : fields_) {
            Source fsrc = f->source();
            src.append(fsrc.into_files()).unwrap();
            writeln_(ss, "#include <{}>", fsrc.name());
        }
        writeln_(ss, "\ntypedef struct {{");
        for (size_t i = 0; i < fields_.size(); ++i) {
            if (fields_[i]->size().unwrap() > 0) {
                writeln_(ss, "    {} {};", fields_[i]->name(), field_name(i));
            }
        }
        writeln_(ss, "}} {};", name());

        std::string fname = rstd::to_lower(name());
        src.insert(fname, ss.str()).unwrap();
        src.set_name(fname);
        
        return src;
    }
    virtual Source source() const override {
        return source_with_names([](size_t i){ return format_("field{}", i); });
    }
    */
};

} // namespace dyn
