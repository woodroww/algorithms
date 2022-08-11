// in build dir
// cmake --build . && ./tests

#include <catch2/catch_test_macros.hpp>

TEST_CASE("false is flase")
{
    REQUIRE(false == false);
}

TEST_CASE("True is false")
{
    REQUIRE(true == false);
}

TEST_CASE("1 + 1 == 2")
{
    REQUIRE(1+1 == 2);
}
