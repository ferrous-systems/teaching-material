#include "hello.h"
#include <stdio.h>

uint32_t cool_printf_wrapper(const char *str)
{
    printf("Cool: %s\n", str);
    return 0; // Everything cool
}