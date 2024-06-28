cmake_minimum_required(VERSION 3.20)
project(scrap_binding C)

set(CMAKE_C_STANDARD 11)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_BUILD_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_BUILD_CMD cargo build --releas-lto)
    set(TARGET_DIR "release-lto")
endif ()

set(SCRAP_LIB_DIR "${PROJECT_SOURCE_DIR}/target/${TARGET_DIR}")

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(OUTPUT_LIB_PATH "${SCRAP_LIB_DIR}/libscrap_binding.so")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    set(OUTPUT_LIB_PATH "${SCRAP_LIB_DIR}/libscrap_binding.dylib")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(OUTPUT_LIB_PATH "${SCRAP_LIB_DIR}/scrap_binding.dll")
else ()
    message(FATAL_ERROR "Unsupported platform")
endif ()

add_library(scrap_binding STATIC lib.c)

add_custom_command(
        TARGET scrap_binding
        PRE_LINK
        COMMAND cargo clean
        COMMAND ${CARGO_BUILD_CMD}
        WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
        COMMENT "Building scrap_binding lib."
)

target_compile_definitions(scrap_binding PUBLIC -DSCRAP_DLL)
target_include_directories(scrap_binding PUBLIC "${PROJECT_SOURCE_DIR}/include")

if (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    target_link_libraries(scrap_binding PRIVATE "${OUTPUT_LIB_PATH}.lib")
else ()
    target_link_libraries(scrap_binding PRIVATE ${OUTPUT_LIB_PATH})
endif ()