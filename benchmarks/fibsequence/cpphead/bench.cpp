#include <iostream>
#include <algorithm>
#include <vector>
#include <functional>
#include <iostream>

extern "C" {
    void start_rapl(){
        std::cout << "Start RAPL" << std::endl;
    }
    void stop_rapl(){
        std::cout << "Stop RAPL" << std::endl;
    }
}

// test method
unsigned long long int fibonacci(unsigned long long int n) {
    if (n <= 1){
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main(int argc, char *argv[]) {
    int fib_param = std::atoi(argv[2]);
    int count = std::atoi(argv[1]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        unsigned long long int result = fibonacci(fib_param);

        stop_rapl();

        // stopping compiler optimization
        if (result < 42){
            std::cout << "Result: " << result << std::endl;
        }
    }

    return 0;
}
