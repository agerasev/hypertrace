#pragma once

#include <rstd.hpp>
#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename ...Types>
class StaticTuple : public Type {
public:
    class Instance : public Type::Instance {
    public:
        rs::Tuple<rs::Box<typename Types::Instance>...> fields_;

        Instance() = default;
        explicit Instance(rs::Box<typename Types::Instance> &&...flds) :
            fields_(std::forward<rs::Box<typename Types::Instance>>(flds)...)
        {}
        explicit Instance(rs::Tuple<rs::Box<typename Types::Instance>...> flds) :
            fields_(std::move(flds))
        {}

        rs::Tuple<rs::Box<typename Types::Instance>...> &fields() {
            return fields_;
        }
        const rs::Tuple<rs::Box<typename Types::Instance>...> &fields() const {
            return fields_;
        }

    private:
        struct TypeExtractor {
            rs::Tuple<rs::Box<Types>...> operator()(const rs::Box<typename Types::Instance> &...insts) {
                return rs::Tuple(insts->type()...);
            }
        };
    public:
        StaticTuple type_() const {
            return StaticTuple(fields_.unpack_ref(TypeExtractor()));
        }
        virtual StaticTuple *_type() const override {
            return new StaticTuple(type_());
        }
        rs::Box<StaticTuple> type() const {
            return rs::Box<StaticTuple>::_from_raw(_type());
        }


    private:
        struct Storer {
            uchar *dst;
            std::vector<size_t> ofv;
            template <size_t P>
            void operator()(const rs::Box<rs::nth_type<P, typename Types::Instance...>> &t) {
                t->store(dst + ofv[P]);
            }
        };

    public:
        virtual void store(uchar *dst) const override {
            std::vector<size_t> ofv = type_().offsets();
            fields_.visit_ref(Storer{dst, ofv});
        }
        virtual void load(const uchar *src) override {
            *this = type_().load_(src);
        }
    };

private:
    rs::Tuple<rs::Box<Types>...> fields_;

public:
    StaticTuple() = default;
    explicit StaticTuple(rs::Box<Types> &&...flds) :
        fields_(std::forward<rs::Box<Types>>(flds)...)
    {}
    explicit StaticTuple(rs::Tuple<rs::Box<Types>...> flds) :
        fields_(std::move(flds))
    {}

    rs::Tuple<rs::Box<Types>...> &fields() {
        return fields_;
    }
    const rs::Tuple<rs::Box<Types>...> &fields() const {
        return fields_;
    }

private:
    struct Cloner {
        rs::Tuple<rs::Box<Types>...> operator()(const rs::Box<Types> &...ts) {
            return rs::Tuple(ts->clone()...);
        }
    };

    struct Hasher {
        rs::DefaultHasher *hasher;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            hasher->_hash_raw(t->id());
        }
    };

public:
    StaticTuple clone_() const {
        return StaticTuple(fields_.unpack_ref(Cloner()));
    }
    virtual StaticTuple *_clone() const override {
        return new StaticTuple(clone_());
    }
    rs::Box<StaticTuple> clone() const {
        return rs::Box<StaticTuple>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(typeid(StaticTuple).hash_code());
        fields_.visit_ref(Hasher{&hasher});
        return hasher.finish();
    }

private:
    struct OffsetCounter {
        std::vector<size_t> *ofv;
        size_t p = 0;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            size_t of = upper_multiple(t->align(), p);
            p = of + t->size().unwrap();
            ofv->push_back(of);
        }
    };
    struct AlignCounter {
        size_t *p;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            *p = std::max(t->align(), *p);
        }
    };
    struct SizeCounter {
        size_t *p;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            *p = upper_multiple(t->align(), *p) + t->size().unwrap();
        }
    };

public:
    std::vector<size_t> offsets() const {
        std::vector<size_t> ofv;
        fields_.visit_ref(OffsetCounter{&ofv});
        return ofv;
    }
    virtual size_t align() const override {
        size_t p = 0;
        fields_.visit_ref(AlignCounter{&p});
        return p;
    }
    virtual rs::Option<size_t> size() const override {
        size_t p = 0;
        fields_.visit_ref(SizeCounter{&p});
        return rs::Some(std::max(upper_multiple(align(), p), (size_t)1));
    }

private:
    struct Loader {
        rs::Tuple<rs::Box<typename Types::Instance>...> *ts;
        const uchar *src;
        std::vector<size_t> ofv;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            ts->template get<P>() = t->load(src + ofv[P]);
        }
    };

public:
    Instance load_(const uchar *src) const {
        rs::Tuple<rs::Box<typename Types::Instance>...> insts;
        fields_.visit_ref(Loader{&insts, src, offsets()});
        return Instance(std::move(insts));
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("StaticTuple{}", id());
    }

private:
    struct Includer {
        std::stringstream *ss;
        Source *src;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            Source fsrc = t->source();
            src->append(fsrc.into_files()).unwrap();
            writeln_(*ss, "#include <{}>", fsrc.name());
        }
    };
    template <typename F>
    struct FieldWriter {
        std::stringstream *ss;
        F field_name;
        template <size_t P>
        void operator()(const rs::Box<rs::nth_type<P, Types...>> &t) {
            if (t->size().unwrap() > 0) {
                writeln_(*ss, "    {} {};", t->name(), field_name(P));
            }
        }
    };

public:
    template <typename F>
    Source source_with_names(F field_name) const {
        std::stringstream ss;
        Source src;

        fields_.visit_ref(Includer{&ss, &src});
        writeln_(ss, "\ntypedef struct {{");
        fields_.visit_ref(FieldWriter<F>{&ss, field_name});
        writeln_(ss, "}} {};", name());

        std::string fname = rs::to_lower(name());
        src.insert(fname, ss.str()).unwrap();
        src.set_name(fname);
        
        return src;
    }
    virtual Source source() const override {
        return source_with_names([](size_t i){ return format_("field{}", i); });
    }
};

} // namespace dyn
