#define LIB_IO
#include "lib.h"

void main() {
    char buf[1024];

    unsigned char x = read_one_byte();
    unsigned char y = read_one_byte();
    char secret = read_one_byte();

    char* a = buf + x;
    char* b = buf + 64 + y;

    a[x] = secret;
    char s = b[x];

    write_one_byte(s);
}
