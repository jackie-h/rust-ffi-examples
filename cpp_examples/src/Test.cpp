#include <test.h>
#include <string>


StringTest createStringTest() {
    StringTest * test = new StringTest();
    //char * name = "Betty";
    //test->name = new char[6];
    //strcpy(test->name, name);
    test->name = "Betty";
    return *test;
}

NestedTest createNestedTest() {
    NestedTest * test = new NestedTest();
    test->stringTest = createStringTest();
    return *test;
}

PointerTest createPointerTest() {
    PointerTest * test = new PointerTest();
    StringTest stringTest = createStringTest();
    test->pointer = &stringTest;
    return *test;
}

UnionAndStringTest createUnionAndStringTest() {
    UnionAndStringTest * test = new UnionAndStringTest();
    UnionTest *unionTest = new UnionTest();
    //unionTest->number = 7.0;
    StringTest * name = new StringTest();
    name->name = "Fred";
    unionTest->pointer = name;
    test->unionTest = *unionTest;
    StringTest stringTest = createStringTest();
    test->stringTest = stringTest;
    return *test;
}