#include <float.h>
#include <stdio.h>

int main() {
    printf("size of char is %zd byte\n", sizeof(char));
    printf("size of short is %zd bytes\n", sizeof(short));
    printf("size of int is %zd bytes\n", sizeof(int));
    printf("size of long is %zd bytes\n", sizeof(long));
    printf("size of __int128_t is %zd bytes\n", sizeof(__int128_t)); // only on 64 bit systems

    printf("\n");

    printf("size of float is %zd bytes\n", sizeof(float));
    printf("size of double is %zd bytes\n", sizeof(double));
    printf("size of long double is %zd bytes\n", sizeof(long double));

    printf("\n");

    printf("maximum value of float: %f\n", FLT_MAX);
    printf("maximum value of double: %f\n", DBL_MAX);
    // printf("maximum value of long double: %Lf\n", LDBL_MAX);

    return 0;
}
