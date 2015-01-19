#include "exampleC.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>


void test_addTwoInt32() {
    assert(addTwoInt32(5, 6) == 11);
}


void test_addTwoDoubles() {
    long result = addTwoDoubles(5.5, 6.5);
    assert(result > 11.9 && result < 12.1);
}


int add3(int num) {
    return num + 3;
}


void test_sendTenToFunction() {
    assert(sendTenToFunction(add3) == 13);
}


void test_getAString() {
    char *str = getAString();
    assert(strnlen(str, 5) > 0);
}


void test_takeAString() {
    char *str = "This is a string";
    assert(takeAString(str) > 0);
}


void test_totalNumbers() {
    int length = 5;
    int numbers[length];
    numbers[0] = 10;
    numbers[1] = 20;
    numbers[2] = 30;
    numbers[3] = 40;
    numbers[4] = 50;
    assert(totalNumbers(length, numbers) == 150);
}


int main(int argc, char *argv[]) {
    printf("Running tests...\n");
    test_addTwoInt32();
    test_addTwoDoubles();
    test_sendTenToFunction();
    test_getAString();
    test_takeAString();
    test_totalNumbers();
    printf("All done!\n");
    return 0;
}
