import java.lang.foreign.Linker;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.ValueLayout;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.Arena;

import java.lang.invoke.MethodHandle;

// java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 .\benchmarks\fibjava\Test.java 10

class Test {
    public static void main(String[] args) {

        System.loadLibrary("rapl_rust_lib");

        MemorySegment start_rapl_symbol = SymbolLookup.loaderLookup().find("start_rapl").get();
        MethodHandle start_rapl_test = Linker.nativeLinker().downcallHandle(start_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));
/*
        MemorySegment stop_rapl_symbol = SymbolLookup.loaderLookup().find("stop_rapl").get();
        MethodHandle stop_rapl_test = Linker.nativeLinker().downcallHandle(stop_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));
*/

        try (Arena arena = Arena.ofConfined()) {
            start_rapl_test.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }

/*
        try (Arena arena = Arena.ofConfined()) {
            stop_rapl_test.invoke();
        } catch (Throwable e) {
            e.printStackTrace();
        }
*/

        int n = Integer.parseInt(args[0]);
        int result = fib(n);
        System.out.println(result);
    }

    public static int fib(int n) {
        if (n < 2) {
            return n;
        }
        return fib(n-1) + fib(n-2);
    }
}