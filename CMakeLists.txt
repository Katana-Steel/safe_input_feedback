cmake_minimum_required(VERSION 4.1)
project(SIF CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(${PROJECT_NAME} OBJECT
    src/readline.cpp
    src/update_feedback.cpp
)
set_target_properties(${PROJECT_NAME} PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_library(${PROJECT_NAME}_shared SHARED $<TARGET_OBJECTS:${PROJECT_NAME}>)
add_library(${PROJECT_NAME}_static STATIC $<TARGET_OBJECTS:${PROJECT_NAME}>)
set_target_properties(${PROJECT_NAME}_shared PROPERTIES OUTPUT_NAME ${PROJECT_NAME})
set_target_properties(${PROJECT_NAME}_static PROPERTIES OUTPUT_NAME ${PROJECT_NAME})

target_link_libraries(${PROJECT_NAME}_shared readline)

add_executable(${PROJECT_NAME}_app
    src/main.cpp
)

target_include_directories(${PROJECT_NAME} PUBLIC include)
target_include_directories(${PROJECT_NAME}_app PUBLIC include)

target_link_libraries(${PROJECT_NAME}_app ${PROJECT_NAME}_shared)

set_target_properties(${PROJECT_NAME} PROPERTIES
    PUBLIC_HEADER "include/readline.hpp;include/readline.h;include/update_feedback.hpp"
)

install(TARGETS ${PROJECT_NAME}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/katana_steel_sif
)
