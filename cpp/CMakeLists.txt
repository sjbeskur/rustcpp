cmake_minimum_required(VERSION 3.16)

project(cppoxide VERSION 1.0.0 DESCRIPTION "test project to call cpp from rust")

# void the use of file(GLOB ...). This feature does not provide attended 
# mastery of the compilation process. If you are lazy, copy-paste output of:
# ls -1 src/*.cpp
add_library(cppoxide SHARED src/rustoxide.cpp )

#target_include_directories(mylib PRIVATE .)
#  or
target_include_directories(cppoxide PRIVATE include)
target_include_directories(cppoxide PRIVATE src)

set_target_properties(cppoxide PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(cppoxide PROPERTIES PUBLIC_HEADER "include/rustoxide.hpp")
#set_target_properties(mylib PROPERTIES SOVERSION 1)

include(GNUInstallDirs)

message("--->" ${CMAKE_INSTALL_INCLUDEDIR} )


install(TARGETS cppoxide
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/cppoxide )