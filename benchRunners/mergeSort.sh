mergeInput="[1,2,3,32,2,3,33,3,4,51,5,5,4,35,345,4,5,2,5,5,3,3,1,2,3,45,5,3,4,4,2,2,55,2,4,4,2,1,12,3,5,5,3,2,5,5,3,2,1,5,4,3,3,2,5,5,2,5,6,5,4,5,5,4,3,2,2,4,5,5,6,4,4,5,6,3,4,2]"
count=1000
testName="mergeSort"
folder="MergeSort"

echo "starting mergeSort"

#   Node
node ./benchmarks/$folder/bench.js $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"

#   Python
python3 ./benchmarks/$folder/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"

#   Pypy
pypy ./benchmarks/$folder/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"

#   C#
dotnet run --project ./benchmarks/$folder/benchC#/Fib.csproj --configuration Release $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/fibjava/Bench.java $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
