// in build dir
// cmake --build . && ./tests

// So the std::vector seems like it is sorting differently if the scores are 
// equal and this causes the test to fail sometimes
// so thats not great for testing

#include <catch2/catch_test_macros.hpp>
#include <vector>
#include "high_scores.h"
#include <iostream>
#include <algorithm>
#include <random>

void print_all_chars(const std::string& stringy) {

	for (auto & c : stringy) {
		if(c=='\n')
			std::cout << "\\n";
		else if(c=='\r')
			std::cout << "\\r";
		else if(c=='\b')
			std::cout << "\\b";
		else if(c=='\t')
			std::cout << "\\t";
		else if(c=='\a')
			std::cout << "\\a";
		else if(c=='\v')
			std::cout << "\\v";
		else if(c=='\0')
			std::cout << "\\0";
		else
			std::cout << c;
	}
}

void print_scores(
	std::vector<GameEntry> true_entries,
	std::vector<GameEntry> test_entries) {

	std::cout << "true ";
	for (auto & i : true_entries) {
		print_all_chars(i.getName());
		std::cout << ":";
		print_all_chars(std::to_string(i.getScore()));
		std::cout << " ";
	}
	std::cout << std::endl;

	std::cout << "test ";
	for (auto & i : test_entries) {
		print_all_chars(i.getName());
		std::cout << ":";
		print_all_chars(std::to_string(i.getScore()));
		std::cout << " ";
	}
	std::cout << std::endl;
}
// random things
// /Users/matt/Documents/Programming/cpp/qt_layout/ImageViewer.cpp

std::vector<GameEntry> setup_true_vector(int count) {
	std::vector<GameEntry> true_entries;
    std::random_device rd; // obtain a random number from hardware
    std::mt19937 gen(rd()); // seed the generator
	std::uniform_int_distribution<> distr(0, 500);
	for (int i = 0; i < count; i++) {
		true_entries.push_back(GameEntry(std::to_string(distr(gen)), distr(gen)));
	}
	return true_entries;
}

TEST_CASE("See if entry is ok")
{
	GameEntry entry = GameEntry("Matt", 45);
    REQUIRE(entry.getScore() == 45);
    REQUIRE(entry.getName() == "Matt");
}

TEST_CASE("One entry")
{
	GameEntry entry = GameEntry("Matt", 45);
	Scores scores = Scores();
	scores.add(entry);
	std::vector<GameEntry> score_vec = scores.getScores();
    REQUIRE(entry.getScore() == score_vec.begin()->getScore());
    REQUIRE(entry.getName() == score_vec.begin()->getName()); 
}

//	std::sort(true_entries.begin(), true_entries.end());
//	std::reverse(true_entries.begin(), true_entries.end());
//  or
//  std::sort(true_entries.rbegin(), true_entries.rend());

TEST_CASE("Ten entries")
{
	std::vector<GameEntry> true_entries = setup_true_vector(10);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	std::sort(true_entries.rbegin(), true_entries.rend());
	std::vector<GameEntry> test_entries = scores.getScores();

	print_scores(true_entries, test_entries);
	REQUIRE(std::equal(true_entries.begin(), true_entries.end(), test_entries.begin()));
}

TEST_CASE("Eleven entries")
{
	std::vector<GameEntry> true_entries = setup_true_vector(11);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	std::sort(true_entries.rbegin(), true_entries.rend());
	true_entries.pop_back();
	std::vector<GameEntry> test_entries = scores.getScores();
	print_scores(true_entries, test_entries);
	REQUIRE(std::equal(true_entries.begin(), true_entries.end(), test_entries.begin()));
}


TEST_CASE("Remove 0 index")
{
	std::vector<GameEntry> true_entries = setup_true_vector(10);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	std::sort(true_entries.rbegin(), true_entries.rend());
	true_entries.erase(true_entries.begin());

	scores.remove(0);

	std::vector<GameEntry> test_entries = scores.getScores();
	print_scores(true_entries, test_entries);
	REQUIRE(std::equal(true_entries.begin(), true_entries.end(), test_entries.begin()));
}

TEST_CASE("Should throw on invalid index")
{
	std::vector<GameEntry> true_entries = setup_true_vector(5);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	REQUIRE_THROWS(scores.remove(5));

	std::vector<GameEntry> true_entries2 = setup_true_vector(11);
	Scores scores2 = Scores();
	for (auto & i : true_entries) {
		scores2.add(i);
	}
	REQUIRE_THROWS(scores2.remove(10));
}

TEST_CASE("Remove last index")
{
	std::vector<GameEntry> true_entries = setup_true_vector(10);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	std::sort(true_entries.rbegin(), true_entries.rend());

	scores.remove(9);
	true_entries.erase(true_entries.begin() + 9);

	std::vector<GameEntry> test_entries = scores.getScores();
	REQUIRE(std::equal(true_entries.begin(), true_entries.end(), test_entries.begin()));
}

TEST_CASE("Remove a middle index")
{
	std::vector<GameEntry> true_entries = setup_true_vector(10);

	auto rng = std::default_random_engine {};
	std::shuffle(std::begin(true_entries), std::end(true_entries), rng);
	Scores scores = Scores();
	for (auto & i : true_entries) {
		scores.add(i);
	}
	std::sort(true_entries.rbegin(), true_entries.rend());

	scores.remove(3);
	true_entries.erase(true_entries.begin() + 3);

	std::vector<GameEntry> test_entries = scores.getScores();
	REQUIRE(std::equal(true_entries.begin(), true_entries.end(), test_entries.begin()));
}



