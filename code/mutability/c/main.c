#include <stdio.h>

int main()
{
    const int x = 5;
    printf("The value of x is %d\n", x);
    *(int *)&x = 6;
    printf("The value of x is %d\n", x);
    return 0;
}
