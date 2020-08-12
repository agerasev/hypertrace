#include <fstream>
#include <sstream>
#include <regex>
#include <cassert>

#include "include.hpp"

#include <iostream>

c_includer::_branch::_branch(int p) : pos(p) {
    
}
c_includer::_branch::~_branch() {
    for(_branch *b : inner) {
        delete b;
    }
}
c_includer::_branch *c_includer::_branch::add(int p) {
    _branch *b = new _branch(p);
    b->parent = this;
    inner.push_back(b);
    return b;
}
void c_includer::_branch::pop() {
    assert(inner.size() > 0);
    delete inner.back();
    inner.pop_back();
}

std::string c_includer::_branch::dir() const {
    size_t dpos = 0;
    for (size_t i = 0; i < fullname.size(); ++i) {
        if (fullname[i] == '/') {
            dpos = i;
        }
    }
    return fullname.substr(0, dpos);
}

std::unique_ptr<std::istream> c_includer::_open(
    const std::string &fullname
) const {
    auto i = _fmem.find(fullname);

    if (i != _fmem.end()) {
        return std::make_unique<std::stringstream>(i->second);
    } else {
        return std::make_unique<std::ifstream>(fullname);
    }
}
std::pair<std::unique_ptr<std::istream>, std::string> c_includer::_find(
    const std::string &name,
    const std::string &dir
) const {
    std::unique_ptr<std::istream> file;
    std::string fullname;

    // try open file in each dir

    // relative include
    if (dir.size() > 0) {
        fullname = dir + "/" + name;
        file = _open(fullname);
    }
    // include from dirs specified
    if(dir.size() == 0 || !(file && *file)) {
        for(const std::string &dir : _dirs) {
            fullname = dir + "/" + name;
            file = _open(fullname);
            if(*file) {
                break;
            }
        }
    }
    // global include
    if (!(file && *file)) {
        fullname = name;
        file = _open(fullname);
    }
    assert(file && *file);

    return std::make_pair(std::move(file), std::move(fullname));
}

bool c_includer::_read(const std::string name, _branch *branch, int depth) {
    assert(depth < 16);
    
    auto p = _find(
        name,
        branch->parent != nullptr ? branch->parent->dir() : ""
    );
    std::unique_ptr<std::istream> file = std::move(p.first);
    std::string fullname = std::move(p.second);
    
    if(!*file) {
        if (branch->parent != nullptr) {
            _log += "\n" + (
                branch->parent->fullname + ":" +
                std::to_string(branch->parent->lsize + 1) + ": " +
                "cannot open file '" + name + "'"
            );
        } else {
            _log += "\n" + ("cannot open file '" + name + "'");
        }
        return false;
    }

    for(const std::string &n : _ignore) {
        if(n == fullname) {
            return false;
        }
    }

    branch->name = name;
    branch->fullname = fullname;
    
    // read file line by line
    std::string line;
    std::smatch match;
    std::regex include("^[ \t]*#include[ ]*[\"<]([^ ]*)[\">]"), pragma("^[  \t]*#pragma[ ]*([^ \t\n]*)");
    while(std::getline(*file, line)) {
        if(std::regex_search(line, match, include)) {
            _branch *b = branch->add(branch->pos + branch->size);
            if (_read(std::string(match[1]), b, depth + 1)){
                branch->size += b->size;
            } else {
                branch->pop();
            }
        } else if(std::regex_search(line, match, pragma)) {
            std::string keyword(match[1]);
            if(keyword == "once") {
                _ignore.push_back(fullname);
            }
        } else {
            _data += line;
        }
        _data += "\n";
        branch->size += 1;
        branch->lsize += 1;
    }

    return true;
}

bool c_includer::_locate(int gp, const _branch *br, std::string &fn, int &lp) const {
    if(gp < br->pos || gp >= br->pos + br->size) {
        return false;
    }
    int shift = gp - br->pos;
    for(const _branch *b : br->inner) {
        if(gp < b->pos) {
            break;
        }
        if(_locate(gp, b, fn, lp)) {
            return true;
        }
        shift -= b->size;
    }
    lp = shift;
    fn = br->fullname;
    return true;
}

c_includer::c_includer(
    const std::string &name,
    const std::list<std::string> &dirs,
    const std::map<std::string, std::string> &fmem
):
    _name(name), _dirs(dirs), _fmem(fmem), _trunk(0)
{}

bool c_includer::include() {
    return _read(_name, &_trunk);
}

const std::string &c_includer::log() const {
    return _log;
}

const std::string &c_includer::data() const {
    return _data;
}

bool c_includer::locate(int gpos, std::string *fullname, int *lpos) const {
    return _locate(gpos, &_trunk, *fullname, *lpos);
}

std::string c_includer::convert(const std::string &message) const {
    std::string result;
    std::string string(message);
    std::regex expr("([^\\s]+):(\\d+):(\\d+):");
    std::smatch match;
    
    while(std::regex_search(string, match, expr)) {
        std::string filename;
        int lpos;
        result += match.prefix().str();
        if (locate(std::stoi(std::string(match[2])), &filename, &lpos)) {
            result += filename + ":" + std::to_string(lpos);
        } else {
            result += std::string(match[1]) + ":" + std::string(match[2]);
        }
        string = ":" + std::string(match[3]) + ":" + match.suffix().str();
        
    }
    result += string;
    
    return result;
}

#ifdef TEST_UNIT

#include <catch.hpp>

TEST_CASE("C Includer", "[include]") {
    SECTION("No include") {
        std::string main = "\
int main() {\n\
    return 0;\n\
}\n\
";
        c_includer includer(
            "main.c",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("main.c"), main)
            }
        );

        REQUIRE(includer.include());
        REQUIRE(includer.data() == main);
    }

    SECTION("Single include") {
        std::string main = "\
#include <header.h>\n\
#include <header.h>\n\
// Main function\n\
int main() {\n\
    return RET_CODE;\n\
}\n\
";
        std::string header = "\
#pragma once\n\
// Return code\n\
static const int RET_CODE = 0;\n\
";

        std::string result = "\
\n\
// Return code\n\
static const int RET_CODE = 0;\n\
\n\
\n\
// Main function\n\
int main() {\n\
    return RET_CODE;\n\
}\n\
";
        c_includer includer(
            "main.c",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("main.c"), main),
                std::make_pair(std::string("header.h"), header)
            }
        );

        REQUIRE(includer.include());
        REQUIRE(includer.data() == result);
    }

    SECTION("Recursion") {
        std::string first = "\
#pragma once\n\
#include <second.h>\n\
";
        std::string second = "\
#pragma once\n\
#include <first.h>\n\
";

        c_includer includer(
            "first.h",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("first.h"), first),
                std::make_pair(std::string("second.h"), second)
            }
        );

        REQUIRE(includer.include());
        REQUIRE(includer.data() == "\n\n\n\n");
    }

    SECTION("Multiple headers") {
        std::string main = "\
#include <h02.h>\n\
#include <h01.h>\n\
";
        std::string h01 = "\
#pragma once\n\
#include <h02.h>\n\
h01\n\
";
        std::string h02 = "\
#pragma once\n\
#include <h01.h>\n\
h02\n\
";

        c_includer includer(
            "main.c",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("main.c"), main),
                std::make_pair(std::string("h01.h"), h01),
                std::make_pair(std::string("h02.h"), h02)
            }
        );

        REQUIRE(includer.include());
        REQUIRE(includer.data() == "\n\n\nh01\n\nh02\n\n\n");
    }

    SECTION("Multiple headers") {
        std::string main = "\
0\n\
1\n\
2\n\
#include <h01.h>\n\
9\n\
10\n\
#include <h03.h>\n\
15\n\
16\n\
";
        std::string h01 = "\
3\n\
#include <h02.h>\n\
7\n\
";
        std::string h02 = "\
4\n\
5\n\
";
        std::string h03 = "\
11\n\
12\n\
13\n\
";

        c_includer includer(
            "main.c",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("main.c"), main),
                std::make_pair(std::string("h01.h"), h01),
                std::make_pair(std::string("h02.h"), h02),
                std::make_pair(std::string("h03.h"), h03)
            }
        );

        REQUIRE(includer.include());
        std::list<std::string> lines;
        std::string accum;
        for (char c : includer.data()) {
            if (c == '\n') {
                lines.push_back(accum);
                accum.clear();
            } else {
                accum += c;
            }
        }
        lines.push_back(accum);
        int i = 0;
        for (const std::string &line : lines) {
            if (line.size() > 0) {
                REQUIRE(std::stoi(line) == i);
            }
            i += 1;
        }
    }

    SECTION("Multiple headers") {
        std::string main = "\
00\n\
01\n\
02\n\
#include <h01.h>\n\
04\n\
05\n\
#include <h03.h>\n\
07\n\
08\n\
";
        std::string h01 = "\
10\n\
#include <h02.h>\n\
12\n\
";
        std::string h02 = "\
20\n\
21\n\
";
        std::string h03 = "\
30\n\
31\n\
32\n\
";

        c_includer includer(
            "main.c",
            std::list<std::string>{},
            std::map<std::string, std::string>{
                std::make_pair(std::string("main.c"), main),
                std::make_pair(std::string("h01.h"), h01),
                std::make_pair(std::string("h02.h"), h02),
                std::make_pair(std::string("h03.h"), h03)
            }
        );

        REQUIRE(includer.include());
        std::list<std::string> lines;
        std::string accum;
        for (char c : includer.data()) {
            if (c == '\n') {
                lines.push_back(accum);
                accum.clear();
            } else {
                accum += c;
            }
        }
        assert(accum.size() == 0);
        int gpos = 0;
        for (const std::string &line : lines) {
            std::string name;
            int lpos;
            REQUIRE(includer.locate(gpos, &name, &lpos));
            if (line.size() > 0) {
                REQUIRE(line.size() == 2);
                char f = line[0];
                int l = std::stoi(line.substr(1, 1));
                if (f == '0') {
                    REQUIRE(name == "main.c");
                } else {
                    std::string hn = "h0x.h";
                    hn[2] = f;
                    REQUIRE(name == hn);
                }
                REQUIRE(lpos == l);
            }
            gpos += 1;
        }
        REQUIRE(!includer.locate(gpos, nullptr, nullptr));
    }
};

#endif // TEST_UNIT
