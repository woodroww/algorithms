#include "double_link.h"

void listReverse(DLinkedList& L) {
	DLinkedList T;
	while (!L.empty()) {
		std::string s = L.front();
		L.removeFront();
	}
	while (!T.empty()) {
		std::string s = T.front();
		T.removeFront();
		L.addBack(s);
	}
}

