mergeInput=`cat benchRunners/mergeSortParam` # getting input from file
count=1000
testName="mergeSort"
folder="mergesort"

echo "!!! Starting $testName !!!"

#   Node
node ./benchmarks/$folder/javascript/bench.js $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"

#   Python
python3 ./benchmarks/$folder/python/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"

#   Pypy
pypy ./benchmarks/$folder/python/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"

#   C#
dotnet run --project ./benchmarks/$folder/csharp/MergeSort.csproj --configuration Release $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"

echo "!!! Finished $testName !!!"
