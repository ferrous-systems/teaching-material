#include "stdint.h"
#include "inttypes.h"
#include "calc_ffi.h"
#include "stdio.h"

// macos notes:
// Doesn't work
// `gcc -L. -l:libcalc_ffi.a demo.c -o ./demo`
// Does work (modulo linking)
// `gcc ./demo.c -L. -l:libcalc_ffi.a -o demo`

int main()
{
    ////////////////////////////////
    // Everything at once
    ////////////////////////////////
    char *text = "4 5 +";
    int64_t output = 0;

    printf("Evaluating: '%s'\n", text);

    Result result = parse_and_eval(text, &output);

    if (result == Ok)
    {
        printf("Result: %lli\n", output);
    }
    else
    {
        printf("FAILED: error code: %i\n", result);
    }

    ////////////////////////////////
    // One part at a time
    ////////////////////////////////

    char *text2 = "3 3 * 5 +";

    printf("Evaluating: '%s'\n", text2);

    struct Expr *box_expr = c_parse(text2);

    if (box_expr != NULL)
    {
        printf("Result: Parse OK\n");
    }
    else
    {
        printf("FAILED\n");
    }

    result = c_eval(box_expr, &output);

    if (result == Ok)
    {
        printf("Result: %lli\n", output);
    }
    else
    {
        printf("FAILED: error code: %i\n", result);
    }

    release_expr(box_expr);
    box_expr = NULL;

    return 0;
}

// -L. -ltestlib
