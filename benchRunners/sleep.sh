count=1
testName="sleep"
folder="sleep"

echo "!!! Starting $testName !!!"
echo

#   C
echo --- Starting C ---
gcc benchmarks/sleep/c/bench.c -O3 -o benchmarks/sleep/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/c/bench $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CEmpty"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/sleep/cpp/bench.cpp -O3 -o benchmarks/sleep/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/cpp/bench $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CppEmpty"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/sleep/javascript/bench.js $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "NodeEmpty"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/sleep/python/bench.py $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "PythonEmpty"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/sleep/python/bench.py $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "PypyEmpty"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/sleep/csharp/Sleep.csproj --configuration Release $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpEmpty"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/sleep/java/Bench.java $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "JavaEmpty"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
