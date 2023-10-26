#include <stdio.h>
#include <stdlib.h>

void start_rapl();
void stop_rapl();

// test method
long long fibb(long long a, long long b, int n) {
    return (--n>0)?(fibb(b, a+b, n)):(a);
}

int main(int argc, char *argv[]) {
    int fibParam = atoi(argv[1]);
    int count = atoi(argv[2]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        long long int result = fibb(0,1,fibParam);
        stop_rapl();

        // stopping compiler optimization
        if (result < 42){
            printf("%lld\n", result);
        }
    }
    return 0;
}
