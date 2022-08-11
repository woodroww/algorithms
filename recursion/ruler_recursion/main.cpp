#include <iostream>

void drawOneTick(int length, int tickLabel = -1) {
	for (int i = 0; i < length; i++) {
		std::cout << '-';
	}
	if (tickLabel >= 0) {
		std::cout << " " << tickLabel;
	}
	std::cout << std::endl;
}

void drawTicks(int tickLength) {
	if (tickLength > 0) {
		drawTicks(tickLength - 1);
		drawOneTick(tickLength);
		drawTicks(tickLength - 1);
	}
}

// total length of ruler, number of `-`'s in the major tick
void drawRuler(int inches, int majorLength) {
	drawOneTick(majorLength, 0);
	for (int i = 1; i <= inches; i++) {
		drawTicks(majorLength - 1);
		drawOneTick(majorLength, i);
	}
}


int main() {
	//drawRuler(2, 4);
	drawRuler(1, 5);
	//drawRuler(3, 3);
}
