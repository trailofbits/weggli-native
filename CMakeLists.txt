cmake_minimum_required(VERSION 3.19)

project(weggli_native)

if(NOT((CMAKE_BUILD_TYPE STREQUAL "Release") OR(CMAKE_BUILD_TYPE STREQUAL "Debug")))
    message(FATAL_ERROR "Only Debug and Release builds are supported")
endif()

find_program(CARGO "cargo" REQUIRED)
find_program(CBINDGEN "cbindgen" REQUIRED)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CARGO_PROFILE "release")
else()
    set(CARGO_PROFILE "dev")
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" TARGET_DIR)

add_custom_command(
    OUTPUT
        "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_STATIC_LIBRARY_SUFFIX}"
        "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_SHARED_LIBRARY_SUFFIX}"
    COMMAND "${CARGO}"
    ARGS build --profile "${CARGO_PROFILE}" --target-dir "${CMAKE_BINARY_DIR}"
    DEPFILE "${CMAKE_BINARY_DIR}/target/${TARGET_DIR}/libweggli_native.d"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}")

add_custom_command(OUTPUT "${CMAKE_BINARY_DIR}/weggli.h"
    COMMAND "${CBINDGEN}"
    ARGS -o "${CMAKE_BINARY_DIR}/weggli.h"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}")

add_custom_target(build-all ALL
    DEPENDS
        "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_STATIC_LIBRARY_SUFFIX}"
        "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_SHARED_LIBRARY_SUFFIX}"
        "${CMAKE_BINARY_DIR}/weggli.h")

include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

set(cmake_install_dir ".")

configure_package_config_file("${CMAKE_SOURCE_DIR}/cmake/${PROJECT_NAME}-config.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake"
    INSTALL_DESTINATION "${cmake_install_dir}")

install(
    FILES "${CMAKE_BINARY_DIR}/weggli.h"
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}")

install(
    FILES "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_STATIC_LIBRARY_SUFFIX}"
    DESTINATION "${CMAKE_INSTALL_LIBDIR}")

install(
    FILES "${CMAKE_BINARY_DIR}/${TARGET_DIR}/libweggli_native${CMAKE_SHARED_LIBRARY_SUFFIX}"
    DESTINATION "${CMAKE_INSTALL_BINDIR}")

install(
    FILES
    "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake"
    DESTINATION "${cmake_install_dir}")