// gcc benchmarks/empty/c/bench.c -o benchmarks/empty/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/c/bench

void start_rapl();
void stop_rapl();

void main() {
    start_rapl();
    stop_rapl();
}