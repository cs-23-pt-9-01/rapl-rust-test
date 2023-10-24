fibInput=200000
count=1000
testName="fib"
folder="FibSequence"

echo "starting fib"

#   Node
echo --- Starting Node.js ---
node ./benchmarks/$folder/bench.js $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- Node.js Done ---

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/benchC#/Fib.csproj --configuration Release $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/fibjava/Bench.java $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---

#   C
echo --- Starting C ---
gcc benchmarks/FibSequence/c/bench.c -O3 -o benchmarks/FibSequence/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/FibSequence/c/bench $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "C$testName"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/FibSequence/cpp/bench.cpp -O3 -o benchmarks/FibSequence/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/FibSequence/cpp/bench $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Cpp$testName"
echo --- C++ Done ---
echo
