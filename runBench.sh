# TODO stop servies running on the computer
echo "starting"

# -- fib --
#   Node
echo "starting fib"

node ./benchmarks/FibSequence/bench.js
mv test.csv fib_node.csv

#   Pypy
pypy ./benchmarks/FibSequence/bench.py
mv test.csv fib_pypy.csv

#   C#
# building
cd benchmarks/FibSequence/benchC#
dotnet build # todo --release
cd ../../..

# running
./benchmarks/FibSequence/benchC#/bin/Debug/net7.0/Fib
mv test.csv fib_csharp.csv

# TODO start services again