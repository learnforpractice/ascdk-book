#include <stdint.h>
#include <eosio/eosio.hpp>

using namespace eosio;

extern "C" void say_hello(const char *name, size_t size) {
    print("hello ", std::string(name, size));
}
