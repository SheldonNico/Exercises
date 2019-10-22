#include <gtest/gtest.h>
#include "../src/blah.h"

TEST(TestBlah, OutputTest) {
  EXPECT_EQ(1, 1);
  EXPECT_EQ(2, sum(1, 1));
}
