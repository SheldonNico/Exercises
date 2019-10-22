project(learn_project)
cmake_minimum_required(VERSION 3.0)

SET(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/modules)

# enable tests
enable_testing() # this must in the top-level cmakelisits.txt
add_subdirectory(${PROJECT_SOURCE_DIR}/tests)

include_directories(${PROJECT_SOURCE_DIR}/src)
add_subdirectory(${PROJECT_SOURCE_DIR}/src)
