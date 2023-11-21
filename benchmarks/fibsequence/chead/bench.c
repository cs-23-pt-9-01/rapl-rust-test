#include <stdio.h>
#include <stdlib.h>

void start_rapl(){
    printf("start\n");
}
void stop_rapl(){
    printf("stop\n");
}

// test method
long long int fibb(long long int a){
    if (a <= 1){
        return a;
    }
    return fibb(a-1) + fibb(a-2);
}

int main(int argc, char *argv[]) {
    int fibParam = atoi(argv[2]);
    int count = atoi(argv[1]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        long long int result = fibb(fibParam);
        stop_rapl();

        // stopping compiler optimization
        if (result < 42){
            printf("%lld\n", result);
        }
    }
    return 0;
}
