testName="sleep"
folder="sleep"
count=1

echo "!!! Starting $testName !!!"
echo

#   C
echo --- Starting C ---
gcc benchmarks/sleep/c/bench.c -O3 -o benchmarks/sleep/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/sleep/c/bench $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CSleep"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/sleep/cpp/bench.cpp -O3 -o benchmarks/sleep/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/sleep/cpp/bench $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CppSleep"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/sleep/javascript/bench.js $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "NodeSleep"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/sleep/python/bench.py $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "PythonSleep"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/sleep/python/bench.py $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "PypySleep"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/sleep/csharp/Sleep.csproj --configuration Release $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpSleep"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/sleep/java/Bench.java $count 5
sleep 5s
bash utils/append_to_latest_csv.sh "JavaSleep"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
