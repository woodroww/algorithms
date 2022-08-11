#pragma once
#include <string>

class StringNode {
private:
	std::string elem;
	StringNode* next;

	friend class StringLinkedList;
};

class StringLinkedList {
public:
	StringLinkedList()
		: head(nullptr) {}

	~StringLinkedList() {
		while (!empty()) removeFront();
	}

	bool empty() const {
		return head == nullptr;
	}

	const std::string& front() const {
		return head->elem;
	}

	void addFront(const std::string& e) {
		StringNode* v = new StringNode;
		v->elem = e;
		v->next = head;
		head = v;
	}

	void removeFront() {
		StringNode* old = head;
		head = old->next;
		delete old;
	}

private:
	StringNode* head;
};


template <typename T>
class SNode;

template <typename T>
class SLinkedList {
public:
	SLinkedList<T>()
		: head(nullptr) {}

	~SLinkedList() {
		while (!empty()) removeFront();
	}
	bool empty() const {
		return head == nullptr;
	}
	const std::string& front() const {
		return head->elem;
	}
	void addFront(const std::string& e) {
		SNode<T>* v = new SNode<T>;
		v->elem = e;
		v->next = head;
		head = v;
	}
	void removeFront() {
		SNode<T>* old = head;
		head = old->next;
		delete old;
	}
private:
	SNode<T>* head;
};

template <typename T>
class SNode {
private:
	T elem;
	SNode* next;
	friend class SLinkedList<T>;
};

