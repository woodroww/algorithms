#include "circle_list.h"
#include <iostream>

void print_list(const CircleList& playList) {
	std::vector<Elem> results = playList.getList();
	std::cout << "[";
	for (std::vector<Elem>::iterator i = results.begin(); i < results.end(); i++) {
		if (i != results.end() - 1) {
			std::cout << *i << ", ";
		} else {
			std::cout << *i;
		}
	}
	std::cout << "]\n";
}

int main() {
	CircleList playList;
	playList.add("Staying Alive");
	print_list(playList);
	playList.add("Le Freak");
	print_list(playList);
	playList.add("Jive Talkin");
	print_list(playList);

	playList.advance();
	print_list(playList);
	playList.advance();
	print_list(playList);
	playList.remove();
	print_list(playList);
	playList.add("Disco Inferno");
	print_list(playList);
	

	
	return 0;
}
