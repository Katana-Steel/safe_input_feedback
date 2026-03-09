cmake_minimum_required(VERSION 3.20...4.1)
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

# --- Rust implementation ---
option(BUILD_RUST "Build the Rust implementation" ON)

if(BUILD_RUST)
    find_program(CARGO_EXECUTABLE cargo)
    if(NOT CARGO_EXECUTABLE)
        message(WARNING "cargo not found, skipping Rust build. Install Rust from https://rustup.rs")
    else()
        if(CMAKE_BUILD_TYPE STREQUAL "Release")
            set(CARGO_BUILD_FLAGS --release)
            set(CARGO_PROFILE "release")
        else()
            set(CARGO_BUILD_FLAGS "")
            set(CARGO_PROFILE "debug")
        endif()

        set(RUST_SOURCE_DIR "${CMAKE_SOURCE_DIR}/rust")
        set(RUST_TARGET_DIR "${CMAKE_BINARY_DIR}/rust-target")
        set(RUST_LIB "${RUST_TARGET_DIR}/${CARGO_PROFILE}/libsafe_input_feedback.a")
        set(RUST_BIN "${RUST_TARGET_DIR}/${CARGO_PROFILE}/safe_input_feedback")

        add_custom_command(
            OUTPUT ${RUST_LIB} ${RUST_BIN}
            COMMAND ${CARGO_EXECUTABLE} build ${CARGO_BUILD_FLAGS}
                --target-dir ${RUST_TARGET_DIR}
            WORKING_DIRECTORY ${RUST_SOURCE_DIR}
            DEPENDS
                ${RUST_SOURCE_DIR}/Cargo.toml
                ${RUST_SOURCE_DIR}/src/lib.rs
                ${RUST_SOURCE_DIR}/src/feedback.rs
                ${RUST_SOURCE_DIR}/src/main.rs
            COMMENT "Building Rust library and binary"
        )

        add_custom_target(rust_build ALL DEPENDS ${RUST_LIB} ${RUST_BIN})

        # Import the static library so other CMake targets can link against it
        add_library(${PROJECT_NAME}_rust STATIC IMPORTED GLOBAL)
        set_target_properties(${PROJECT_NAME}_rust PROPERTIES
            IMPORTED_LOCATION ${RUST_LIB}
        )
        add_dependencies(${PROJECT_NAME}_rust rust_build)

        # Copy the Rust binary into the CMake build directory
        add_custom_command(
            TARGET rust_build POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                ${RUST_BIN} ${CMAKE_BINARY_DIR}/SIF_rust_app
            COMMENT "Copying Rust binary to build directory"
        )
    endif()
endif()
