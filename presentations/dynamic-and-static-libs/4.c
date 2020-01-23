#include "../include/point.h"

int main (int argc, char const *argv[])
{
        Point* p = new_point(1,1);
        inspect_point(p);
        p->x = 2;
        inspect_point(p);
        destroy_point(p);
}