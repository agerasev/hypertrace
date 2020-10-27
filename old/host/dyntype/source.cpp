#include "source.hpp"

using namespace rstd;
using namespace dyn;


Result<Tuple<>, std::string> Source::insert(
    const std::string &fname, std::string &&content
) {
    auto it = files_.find(fname);
    if (it == files_.end()) {
        assert_(files_.insert(std::make_pair(fname, std::move(content))).second);
        return Ok();
    } else {
        if (it->second == content) {
            return Ok();
        } else {
            return Err(format_("Same file '{}' but different content", fname));
        }
    }
}
Result<Tuple<>, std::string> Source::append(
    std::map<std::string, std::string> &&other
) {
    files_.merge(other);
    for (const auto &s : other) {
        auto di = files_.find(s.first);
        if (di->second != s.second) {
            return Err(format_("Same file '{}' but different content", s.first));
        }
    }
    return Ok();
}
