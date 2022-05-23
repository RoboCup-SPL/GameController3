cmake_minimum_required(VERSION 3.15)
project(GameController VERSION 2021.0.0 LANGUAGES CXX)

option(WITH_COVERAGE "Enable coverage." OFF)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Protobuf REQUIRED)

add_library(Flags::Default INTERFACE IMPORTED)

target_compile_options(Flags::Default INTERFACE
    $<$<CXX_COMPILER_ID:Clang,AppleClang,GNU>:-Wall>
    $<$<CXX_COMPILER_ID:Clang,AppleClang,GNU>:-Wextra>
    $<$<CXX_COMPILER_ID:MSVC>:/W3>
)

if(WITH_COVERAGE)
  target_compile_options(Flags::Default INTERFACE
      $<$<CXX_COMPILER_ID:Clang,AppleClang>:-fprofile-instr-generate>
      $<$<CXX_COMPILER_ID:Clang,AppleClang>:-fcoverage-mapping>
      $<$<CXX_COMPILER_ID:GNU>:--coverage>
  )
  target_link_options(Flags::Default INTERFACE
      $<$<CXX_COMPILER_ID:Clang,AppleClang>:-fprofile-instr-generate>
      $<$<CXX_COMPILER_ID:Clang,AppleClang>:-fcoverage-mapping>
      $<$<CXX_COMPILER_ID:GNU>:--coverage>
  )
endif()

add_subdirectory(Core)
add_subdirectory(Leagues/SPL)
