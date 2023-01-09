#include "rustoxide.hpp"

extern "C" float do_something_cool(int one, int two)
{
    return (one + 2) / (two + 2) * 1000;
}

extern "C" std::vector<float> add_to_vector(float x)
{
    std::vector<float> list;
    list.push_back(x);
    return list;
}

extern "C" const char *get_string(int one, int two)
{
    std::string s = std::to_string(one) + " " + std::to_string(two);
    return s.c_str();
}
