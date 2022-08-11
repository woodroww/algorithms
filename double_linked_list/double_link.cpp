#include "double_link.h"

DLinkedList::DLinkedList() {
	header = new DNode;
	trailer = new DNode;
	header->next = trailer;
	trailer->prev = header;
}

DLinkedList::~DLinkedList() {
	while (!empty()) removeFront();
	delete header;
	delete trailer;
}

DLinkedList::empty() const {
	return (header->next == trailer);
}

const Elem& DLinkedList::front() const {
	// throw if list is empty
	return header->next->elem;
}

const Elem& DLinkedList::back() const {
	// throw if list is empty
	return trailer->prev->elem;
}

// add element before v
void DLinkedList::add(DNode* v, const Elem& e) {
	DNode* prev = v->prev;
	DNode* newNode = new DNode;
	newNode->elem = e;
	prev->next = newNode;
	v->prev = newNode;
	newNode->prev = prev;
	newNode->next = v;
}
void DLinkedList::addFront(const Elem& e) {
	add(header->next, e);
}

void DLinkedList::addBack(const Elem& e) {
	add(trailer, e);
}

// remove 
void DLinkedList::remove(DNode* v) {
	DNode* prev = v->prev;
	DNode* next = v->next;
	prev->next = next;
	next->prev = prev;
	delete v;
}

void DLinkedList::removeFront() {
	remove(header->next);
}

void DLinkedList::removeBack() {
	remove(trailer->prev);
}


