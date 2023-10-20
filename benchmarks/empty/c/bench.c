#include <stdio.h>
#include <stdlib.h>

void start_rapl();
void stop_rapl();

void main(int argc, char *argv[]) {
    int count = atoi(argv[1]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        stop_rapl();
    }

    printf("C job done\n");
}