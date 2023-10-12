import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

// OLD:
// java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 .\benchmarks\fibjava\Bench.java 10 10

// Testing with Java library path:
// java -Djava.library.path=./target/release --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/FibSequence/fibjava/Bench.java 10 10

// Latest working version:
// java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/FibSequence/fibjava/Bench.java 10 10

class Bench {
    public static void main(String[] args) {
        var os = System.getProperty("os.name");

        var dll_path = System.getProperty("user.dir") + "/target/release/";
        if (os.equals("Linux")) {
            dll_path = dll_path + "librapl_lib.so";
        } else if (os.equals("Windows 11")) {
            dll_path = dll_path + "rapl_lib.dll";
        } else {
            System.out.println("OS not supported");
            return;
        }

        System.load(dll_path);

        MemorySegment start_rapl_symbol = SymbolLookup.loaderLookup().find("start_rapl").get();
        MethodHandle start_rapl = Linker.nativeLinker().downcallHandle(start_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        MemorySegment stop_rapl_symbol = SymbolLookup.loaderLookup().find("stop_rapl").get();
        MethodHandle stop_rapl = Linker.nativeLinker().downcallHandle(stop_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        int n = Integer.parseInt(args[0]);
        int loop_count = Integer.parseInt(args[1]);

        /*
        // works without arena as seen below, but not sure if it is correct to do so
        // the code is commented out here in case it is needed later

        try (Arena arena = Arena.ofConfined()) {
            start_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }
        */

        // Note that this could potentially be optimized away
        // by the compiler due to the fact that the result is not used.
        for (int i = 0; i < loop_count; i++) {
            try {
                start_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }

            int result = fib(n);

            try {
                stop_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }
        }

        /*
        try (Arena arena = Arena.ofConfined()) {
            stop_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }
        */
    }

    public static int fib(int n) {
        if (n < 2) {
            return n;
        }
        return fib(n-1) + fib(n-2);
    }
}