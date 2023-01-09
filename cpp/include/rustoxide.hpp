#ifndef RUSTCPP_H
#define RUSTCPP_H

#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

extern "C" float do_something_cool(int one, int two);

// fails
extern "C" std::vector<float> add_to_vector(float x);

extern "C" const char *get_string(int one, int two);

#endif