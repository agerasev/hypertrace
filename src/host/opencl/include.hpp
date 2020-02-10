#pragma once

#include <list>
#include <string>
#include <fstream>
#include <exception>
#include <regex>


class c_includer {
private:
	class _branch {
	public:
		int pos;
		int size = 0;
		int lsize = 0;
		std::string fullname;
		std::string name;
		
		std::list<_branch*> inner;
		_branch *parent = nullptr;
		
		_branch(int p) : pos(p) {
			
		}
		~_branch() {
			for(_branch *b : inner) {
				delete b;
			}
		}
		_branch *add(int p) {
			_branch *b = new _branch(p);
			b->parent = this;
			inner.push_back(b);
			return b;
		}
		void pop() {
			assert(inner.size() > 0);
			delete inner.back();
			inner.pop_back();
		}

		std::string dir() const {
			size_t dpos = 0;
			for (size_t i = 0; i < fullname.size(); ++i) {
				if (fullname[i] == '/') {
					dpos = i;
				}
			}
			return fullname.substr(0, dpos);
		}
	};
	
	std::string _name;
	std::list<std::string> _dirs;
	std::string _data;
	
	std::list<std::string> _ignore;
	_branch _trunk;

	std::string _log;
	
	bool _read(const std::string name, _branch *branch, int depth = 0) {
		assert(depth < 16);

		branch->name = name;
		for(const std::string &n : _ignore) {
			if(n == name) {
				return true;
			}
		}
		
		std::ifstream file;
		
		// try open file in each dir
		std::string fullname;
		if (branch->parent != nullptr) {
			fullname = branch->parent->dir() + "/" + name;
			file.open(fullname);
		}
		if(branch->parent == nullptr || !file) {
			for(const std::string &dir : _dirs) {
				fullname = dir + "/" + name;
				file.open(fullname);
				if(file) {
					break;
				}
			}
			if(!file) {
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
		}
		branch->fullname = fullname;
		
		// read file line by line
		std::string line;
		std::smatch match;
		std::regex include("^[ \t]*#include[ ]*[\"<]([^ ]*)[\">]"), pragma("^[  \t]*#pragma[ ]*([^ \t\n]*)");
		while(std::getline(file, line)) {
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
					_ignore.push_back(name);
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
	
	bool _locate(int gp, const _branch *br, std::string &fn, int &lp) const {
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
	
public:
	c_includer(const std::string &name, const std::list<std::string> &dirs):
		_name(name), _dirs(dirs), _trunk(0)
	{}

	bool include() {
		return _read(_name, &_trunk);
	}
	
	const std::string &log() const {
		return _log;
	}

	const std::string &data() const {
		return _data;
	}
	
	bool locate(int gpos, std::string *fullname, int *lpos) const {
		return _locate(gpos, &_trunk, *fullname, *lpos);
	}

	std::string convert(const std::string &message) const {
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
};
