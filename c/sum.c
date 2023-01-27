#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include "bindings.h"

Value sum(Value a, Value b)
{

    a.number += b.number;
    return a;
}