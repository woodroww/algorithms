#include "circle_list.h"

CircleList::CircleList()
	: cursor(nullptr) { }

CircleList::~CircleList() {
	while (!empty()) {
		remove();
	}
}

bool CircleList::empty() const {
	return (cursor == nullptr);
}

const Elem& CircleList::back() const {
	return cursor->elem;
}

const Elem& CircleList::front() const {
	return cursor->next->elem;
}

void CircleList::advance() {
	cursor = cursor->next;
}

void CircleList::add(const Elem& e) {
	CNode* node = new CNode;
	node->elem = e;
	if (empty()) {
		node->next = node;
		cursor = node;
	} else {
		node->next = cursor->next;
		cursor->next = node;
	}
}

// remove node after cursor
void CircleList::remove() {
	CNode* old = cursor->next;
	// if cursor points to itself it is the only node
	if (old == cursor) {
		cursor = nullptr;
	} else {
		cursor->next = old->next;
	}
	delete old;
}

std::vector<Elem> CircleList::getList() const {
	std::vector<Elem> result;
	if (empty()) return result;
	CNode* current = cursor->next;
	bool done = false;
	while (!done) {
		result.push_back(current->elem);
		current = current->next;
		if (current == cursor->next) {
			done = true;
		}
	}
	return result;
}










