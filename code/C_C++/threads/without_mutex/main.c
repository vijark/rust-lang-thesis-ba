#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

pthread_t tid[2];
int count;

void *do_something(void *arg) {
    printf("job %d started\n", ++count);
    sleep(1);
    printf("job %d finished\n", count);

    return NULL;
}

int main() {
    int i = 0;
    while (i < 2) {
        int err = pthread_create(&(tid[i]), NULL, &do_something, NULL);
        if (err) {
            fprintf(stderr, "error while creating thread: %s\n", strerror(err));
        }
        i++;
    }

    pthread_join(tid[0], NULL);
    pthread_join(tid[1], NULL);

    return 0;
}

// job 1 started
// job 2 started
// job 2 finished
// job 2 finished
