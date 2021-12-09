#include <stdio.h>

#include "example.h"

int main(int argc, char **argv) {
  magic_adder_t ma = magicadder_new(5);
  printf("5 + 6 = %u\n", magicadder_process_value(&ma, 6));
  return 0;
}
