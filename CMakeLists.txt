cmake_minimum_required(VERSION 4.1)
project(SIF CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(${PROJECT_NAME} SHARED
    src/readline.cpp
    src/update_feedback.cpp
)

target_include_directories(${PROJECT_NAME} PUBLIC include)

target_link_libraries(${PROJECT_NAME} readline)

add_executable(${PROJECT_NAME}_app
    src/main.cpp
)

target_link_libraries(${PROJECT_NAME}_app ${PROJECT_NAME})
