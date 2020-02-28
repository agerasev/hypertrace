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
		
		_branch(int p);
		~_branch();
		_branch *add(int p);
		void pop();
		std::string dir() const;
	};
	
	std::string _name;
	std::list<std::string> _dirs;
	std::string _data;
	
	std::list<std::string> _ignore;
	_branch _trunk;

	std::string _log;
	
	bool _read(const std::string name, _branch *branch, int depth = 0);
	bool _locate(int gp, const _branch *br, std::string &fn, int &lp) const;
	
public:
	c_includer(const std::string &name, const std::list<std::string> &dirs);

	bool include();
	const std::string &log() const;
	const std::string &data() const;

	bool locate(int gpos, std::string *fullname, int *lpos) const;
	std::string convert(const std::string &message) const;
};
