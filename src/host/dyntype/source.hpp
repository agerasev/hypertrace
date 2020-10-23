#pragma once

#include <string>
#include <map>
#include <rstd/prelude.hpp>

namespace dyn {

class Source {
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
    rstd::Result<rstd::Tuple<>, std::string> insert(
        const std::string &fname, std::string &&content
    );
    rstd::Result<rstd::Tuple<>, std::string> append(
        std::map<std::string, std::string> &&other
    );
};

} // namespace dyn
