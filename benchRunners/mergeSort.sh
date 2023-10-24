mergeInput=`cat benchRunners/mergeSortParam` # getting input from file
count=1000
testName="mergeSort"
folder="MergeSort"

echo "starting mergeSort"

#   Node
echo --- Starting Node.js ---
node ./benchmarks/$folder/bench.js $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- Node.js Done ---

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/benchC#/MergeSort.csproj --configuration Release $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---

#   C
echo --- Starting C ---
gcc benchmarks/MergeSort/c/bench.c -O3 -o benchmarks/MergeSort/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/MergeSort/c/bench $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "C$testName"
echo --- C Done ---

#   C++
echo --- Starting C++ ---
g++ benchmarks/MergeSort/cpp/bench.cpp -O3 -o benchmarks/MergeSort/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/MergeSort/cpp/bench $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Cpp$testName"
echo --- C++ Done ---
