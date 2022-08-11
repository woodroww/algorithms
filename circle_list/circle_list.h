#pragma once
#include <string>
#include <vector>

typedef std::string Elem;
class CNode {
private:
	Elem elem;
	CNode* next;
	friend class CircleList;
};

class CircleList {
public:
	CircleList();
	~CircleList();
	bool empty() const;			// is empty
	const Elem& front() const;	// element at cursor
	const Elem& back() const;	// element following cursor
	void advance();				// advance cursor
	void add(const Elem& e);	// add after cursor
	void remove();				// remove node after cursor
	std::vector<Elem> getList() const;
private:
	CNode* cursor;
};
