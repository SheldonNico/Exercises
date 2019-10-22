add_subdirectory(/usr/src/gtest ${PROJECT_BINARY_DIR}/gtest)

include(CTest) # cmake tools for test, to help gtest
set(TEST_BINARY ${PROJECT_NAME}_test)
add_executable(${TEST_BINARY} test.cpp)
target_link_libraries(${TEST_BINARY} blah gtest gtest_main)
add_test(
  NAME ${TEST_BINARY}
  COMMAND ${EXECUTABLE_OUTPUT_PATH}/${TEST_BINARY}
  )
