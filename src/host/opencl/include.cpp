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
    if(dir.size() == 0 || !*file) {
        for(const std::string &dir : _dirs) {
            fullname = dir + "/" + name;
            file = _open(fullname);
            if(*file) {
                break;
            }
        }
    }
    // global include
    if (!*file) {
        fullname = name;
        file = _open(fullname);
    }

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
