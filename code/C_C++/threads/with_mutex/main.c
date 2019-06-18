#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

pthread_t tid[2];
int count;
pthread_mutex_t lock;

void *do_something(void *arg) {
    pthread_mutex_lock(&lock);

    printf("job %d started\n", ++count);
    sleep(1);
    printf("job %d finished\n", count);

    pthread_mutex_unlock(&lock);

    return NULL;
}

int main() {
    if (pthread_mutex_init(&lock, NULL)) {
        fprintf(stderr, "mutex init failed");
        exit(EXIT_FAILURE);
    }

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
// job 1 finished
// job 2 started
// job 2 finished
