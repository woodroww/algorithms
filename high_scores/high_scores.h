#pragma once
#include <string>
#include <vector>
#include <iterator>
#include <iostream>

class RuntimeException {
public:
	RuntimeException(const std::string& err)
		: errMsg(err) { }
	std::string getError() { return errMsg; }
private:
	std::string errMsg;
};

class IndexOutOfBounds : public RuntimeException {
public:
	IndexOutOfBounds()
		: RuntimeException("Index out of bounds")
	{ }
};

class GameEntry {
public:
	GameEntry(const std::string& n="", int s=0);
	GameEntry(const GameEntry& entry);
	std::string getName() const;
	int getScore() const;
	bool operator < (const GameEntry& entry) const; 
	bool operator > (const GameEntry& entry) const; 
	bool operator == (const GameEntry& entry) const;
    friend std::ostream& operator<<(std::ostream& os, const GameEntry& dt);
private:
	std::string name;
	int score;
};


class Scores {
public:
	Scores(int maxEnt = 10);
	~Scores();
	void add(const GameEntry& e);
	GameEntry remove(int i) throw(IndexOutOfBounds);

	std::vector<GameEntry> getScores() const {
		std::vector<GameEntry> result;
		for (int i = 0; i < numEntries; i++) {
			result.push_back(entries[i]);
		}
		return result;
	};
private:
	int maxEntries;
	int numEntries;
	GameEntry* entries;
};


