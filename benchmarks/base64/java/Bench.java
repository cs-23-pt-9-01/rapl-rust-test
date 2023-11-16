import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

class Bench {
    public static void main(String[] args) {
        // Find os
        var os = System.getProperty("os.name");

        // Find the path of the library (and load it)
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

        // Load functions
        MemorySegment start_rapl_symbol = SymbolLookup.loaderLookup().find("start_rapl").get();
        MethodHandle start_rapl = Linker.nativeLinker().downcallHandle(start_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        MemorySegment stop_rapl_symbol = SymbolLookup.loaderLookup().find("stop_rapl").get();
        MethodHandle stop_rapl = Linker.nativeLinker().downcallHandle(stop_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        // Get arguments
        int loop_count = Integer.parseInt(args[0]);

        // Running benchmark
        // Note that this could potentially be optimized away
        // by the compiler due to the fact that the result is not used.
        for (int i = 0; i < loop_count; i++) {
            try {
                start_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }

            // Do something

            try {
                stop_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }
        }
    }
}