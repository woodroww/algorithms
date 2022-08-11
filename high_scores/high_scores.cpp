#include "high_scores.h"
#include <iostream>

GameEntry::GameEntry(const std::string& n, int s)
	: name(n), score(s) { 
}
GameEntry::GameEntry(const GameEntry& entry)
	: name(entry.getName()), score(entry.getScore()) { }

bool GameEntry::operator > (const GameEntry& entry) const {
	return (getScore() > entry.getScore());
}
bool GameEntry::operator < (const GameEntry& entry) const {
	return (getScore() < entry.getScore());
}
bool GameEntry::operator == (const GameEntry& entry) const {
	if (getScore() != entry.getScore())
		std::cout << "wtf score " << getScore() << " != " << entry.getScore() << std::endl;
	if (getName() != entry.getName())
		std::cout << "wtf name  " << getName() << " != " << entry.getName() << std::endl;
	return ((getScore() == entry.getScore()) && (getName() == entry.getName()));
}
std::string GameEntry::getName() const {
	return name;
}
int GameEntry::getScore() const {
	return score;
}
std::ostream& operator<<(std::ostream& os, const GameEntry& entry) {
	os << entry.getName() << ":" << entry.getScore();
	return os;
}


Scores::Scores(int maxEnt) {
	maxEntries = maxEnt;
	entries = new GameEntry[maxEntries];
	numEntries = 0;
}

Scores::~Scores() {
	delete[] entries;
}

void Scores::add(const GameEntry& e) {
	int newScore = e.getScore();
	if (numEntries == maxEntries) {
		if (newScore <= entries[maxEntries - 1].getScore()) {
			return;
		}
	} else {
		numEntries++;
	}
	int i = numEntries - 2;
	while (i >= 0 && newScore > entries[i].getScore()) {
		entries[i+1] = entries[i]; // shift right if smaller
		i--;
	}
	entries[i+1] = e;
}

GameEntry Scores::remove(int i) throw(IndexOutOfBounds) {
	if ((i < 0) || (i >= numEntries)) {
		throw IndexOutOfBounds();
	}
	GameEntry e = entries[i];
	for (int j = i+1; j < numEntries; j++) {
		entries[j-1] = entries[j];
	}
	numEntries--;
	return e;
}


