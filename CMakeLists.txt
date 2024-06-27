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

set(BUILD_DEP ${PROJECT_SOURCE_DIR}/build.rs)
set(BUILD_OUTPUT ${PROJECT_SOURCE_DIR}/include/scrap_binding.h)

add_library(scrap_binding_dep STATIC lib.c)

add_custom_command(
        TARGET scrap_binding_dep
        PRE_BUILD
        COMMAND cargo clean
        COMMAND ${CARGO_BUILD_CMD}
        WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
        COMMENT "Building scrap_binding lib."
)

add_library(scrap_binding INTERFACE)
target_compile_definitions(scrap_binding INTERFACE -DSCRAP_DLL)
target_include_directories(scrap_binding INTERFACE "${PROJECT_SOURCE_DIR}/include")

set(SCRAP_LIB_DIR "${PROJECT_SOURCE_DIR}/target/${TARGET_DIR}")

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    target_link_libraries(scrap_binding INTERFACE "${SCRAP_LIB_DIR}/libscrap_binding.so")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    target_link_libraries(scrap_binding INTERFACE "${SCRAP_LIB_DIR}/libscrap_binding.dylib")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    target_link_libraries(scrap_binding INTERFACE "${SCRAP_LIB_DIR}/scrap_binding.dll.lib" Ws2_32.lib Userenv.lib D3D11.lib ntdll.lib DXGI.lib)
else ()
    message(FATAL_ERROR "Unsupported platform")
endif ()

target_link_libraries(scrap_binding_dep INTERFACE scrap_binding)