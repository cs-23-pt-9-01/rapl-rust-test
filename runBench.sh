# TODO stop servies running on the computer

# -- fib --
#   Node
node .\\benchmarks\\FibSequence\\bench.js

#   Pypy
Pypy .\\benchmarks\\FibSequence\\bench.py

#   C#
# building
cd benchmarks\\FibSequence\\benchC#
dotnet build # todo --release
cd ..\\..\\..
# running
bash .\\benchmarks\\FibSequence\\benchC#\\bin\\Debug\\net7.0\\Fib.exe