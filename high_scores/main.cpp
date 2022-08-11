#include "high_scores.h"
#include <iostream>

int main() {

	GameEntry entry = GameEntry("Jam", 45);
	std::cout << "Hello, world!\n" << entry.getName() << " ";
	std::cout << entry.getScore() << std::endl;
	return 0;
}
