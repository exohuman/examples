// exampleC.c

#include <string.h>
#include "exampleC.h"

int addTwoInt32(int a, int b) {
    return a + b;
}


double addTwoDoubles(double a, double b) {
    return a + b;
}


int sendTenToFunction(int (*func)(int)) {
    return (*func)(10);
}


char *getAString() {
    return "This is the string being returned.";
}


int takeAString(const char *str) {
    return strnlen(str, 5) > 0;
}


int totalNumbers(int length, int nums[]) {
    int total = 0;
    for (int i = 0; i < length; i++) {
        total += nums[i];
    }
    return total;
}
