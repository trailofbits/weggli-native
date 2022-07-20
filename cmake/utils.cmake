
# Utility function to get native library dependencies
function(find_native_staticlibs_dependencies RUST_LINK_LIBRARIES_CACHED)
  
  if (DEFINED ${RUST_LINK_LIBRARIES_CACHED})
    message(STATUS "Rust link libraries: ${RUST_LINK_LIBRARIES_CACHED}")
    return()
  endif()
  
  find_program(RUSTC "rustc" REQUIRED)
  get_filename_component(RUSTC_NAME "${RUSTC}" NAME)
  
  execute_process(
    COMMAND ${CARGO} ${RUSTC_NAME} -- --print=native-static-libs
    OUTPUT_VARIABLE LINK_LIBRARIES_OUT
    ERROR_VARIABLE LINK_LIBRARIES_ERR
    RESULT_VARIABLE CORGO_RET
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}")

  if (NOT "${CORGO_RET}" STREQUAL "0")
    message(FATAL_ERROR "cargo build failed: ${LINK_LIBRARIES_ERR}")
  endif()
  
  set(RUST_LINK_LIBRARIES "${LINK_LIBRARIES_OUT} ${LINK_LIBRARIES_ERR}")
  string(REGEX MATCHALL "note: native-static-libs: ([\-a-zA-Z_0-9+ \.]+)" RUST_LINK_LIBRARIES "${RUST_LINK_LIBRARIES}")
  string(REPLACE "note: native-static-libs: " "" RUST_LINK_LIBRARIES "${RUST_LINK_LIBRARIES}")
  
  message(STATUS "rust libraries: ${RUST_LINK_LIBRARIES}")
  set(${RUST_LINK_LIBRARIES_CACHED} "${RUST_LINK_LIBRARIES}" CACHE STRING "Required link libraries by rust" FORCE)

endfunction(find_native_staticlibs_dependencies)