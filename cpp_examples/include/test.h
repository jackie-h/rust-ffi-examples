struct StringTest {

    StringTest()
            :
            name( 0 ) {}

public:
    char * name;
};

struct NestedTest {
    StringTest stringTest;

};

struct PointerTest {
    const void * pointer;
};

union UnionTest {
    double number;
    const void * pointer;
};

struct UnionAndStringTest {
    UnionTest unionTest;
    StringTest stringTest;
};

extern "C" {
   StringTest createStringTest();
   NestedTest createNestedTest();
   PointerTest createPointerTest();
   UnionAndStringTest createUnionAndStringTest();
}