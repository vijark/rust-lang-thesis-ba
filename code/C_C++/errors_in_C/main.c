#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define USERNAME_LENGTH 50

void read_username(char *, int);

int main() {
    char name[USERNAME_LENGTH];
    read_username(name, USERNAME_LENGTH);

    printf("The username is: %s.\n", name);

    return 0;
}

void read_username(char *username, int len) {
    char c;
    FILE *fp;

    fp = fopen("username.txt", "r");

    if (fp == NULL) {
        fprintf(stderr, "Something went wrong: %s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }

    c = fgetc(fp);
    int i = 0;
    for (; c != EOF && i < len - 1; ++i, c = fgetc(fp)) {
        username[i] = c;
    }
    username[i + 1] = '\0';

    fclose(fp);
}
