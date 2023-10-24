#include <stdio.h>
#include <stdlib.h>

void start_rapl();
void stop_rapl();

// test method
long long int fibb(int n) {
	int fnow = 0, fnext = 1, tempf;
	while(--n>0){
		tempf = fnow + fnext;
		fnow = fnext;
		fnext = tempf;
	}
	return fnext;	
}

int main(int argc, char *argv[]) {
    int fibParam = atoi(argv[1]);
    int count = atoi(argv[2]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        long long int result = fibb(fibParam);
        stop_rapl();

        // stopping compiler optimization
        if (result < 42){
            printf("Result: %lld\n", result);
        }
    }
    return 0;
}
