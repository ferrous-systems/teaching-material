#include <stdint.h>
#include <stdbool.h>

typedef struct Point {
    int32_t x;
    int32_t y;
} Point;

Point* new_point(int32_t x, int32_t y);

void destroy_point(Point* p);

void inspect(Point* p);