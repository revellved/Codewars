cmake_minimum_required(VERSION 3.1)

project(codewars)

find_package(GTest   REQUIRED)
find_package(Threads REQUIRED)

set(CMAKE_CXX_STANDARD          11)
set(CMAKE_CXX_STANDARD_REQUIRED on)

include_directories(
    ../include
    ${GTEST_INCLUDE_DIRS}
)

add_executable(main test)

target_link_libraries(
  TestSerialization ${GTEST_LIBRARIES} Threads::Threads)

enable_testing()
add_test(TestSerialization "./TestSerialization")
