import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

// run with:
// java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 .\benchmarks\fibjava\Bench.java 10

class Bench {
    public static void main(String[] args) {
        System.loadLibrary("target/release/rapl_lib");

        MemorySegment start_rapl_symbol = SymbolLookup.loaderLookup().find("start_rapl").get();
        MethodHandle start_rapl = Linker.nativeLinker().downcallHandle(start_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        MemorySegment stop_rapl_symbol = SymbolLookup.loaderLookup().find("stop_rapl").get();
        MethodHandle stop_rapl = Linker.nativeLinker().downcallHandle(stop_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        int n = Integer.parseInt(args[0]);

        System.out.println("calling start_rapl");

        try {
            start_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }

        /*
        // works without arena as seen above, but not sure if it is correct to do so
        // the code is commented out here in case it is needed later

        try (Arena arena = Arena.ofConfined()) {
            start_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }
        */

        int result = fib(n);
        System.out.println(result);

        try {
            stop_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }

        /*
        try (Arena arena = Arena.ofConfined()) {
            stop_rapl.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }
        */

        System.out.println("called stop_rapl");
    }

    public static int fib(int n) {
        if (n < 2) {
            return n;
        }
        return fib(n-1) + fib(n-2);
    }
}