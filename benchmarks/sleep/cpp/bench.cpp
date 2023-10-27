#include <iostream>

extern "C" {
    void start_rapl();
    void stop_rapl();
}

int main(int argc, char *argv[]) {
    int count = std::atoi(argv[1]);
    int sleep_time = std::atoi(argv[2]);

    for (int i = 0; i < count; i++) {
        start_rapl();
        sleep(1);
        stop_rapl();
    }

    return 0;
}
