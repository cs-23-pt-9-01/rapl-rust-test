import java.lang.foreign.Linker;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.ValueLayout;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.MemorySegment;

import java.lang.invoke.MethodHandle;

// java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 .\benchmarks\fibjava\Test.java 10

class Test {
    public static void main(String[] args) {

        System.loadLibrary("rapl_rust_lib");

        MemorySegment awer = SymbolLookup.loaderLookup().find("start_rapl").get();

        var linker = Linker.nativeLinker();
        SymbolLookup lookup = linker.defaultLookup();

        MethodHandle start_rapl_test = linker.downcallHandle(lookup.find("start_rapl").get(),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

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