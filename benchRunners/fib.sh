fibInput=200000
count=1000
testName="fib"
folder="fibsequence"

echo "!!! Starting $testName !!!"

#   Node
node ./benchmarks/$folder/javascript/bench.js $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"

#   Python
python3 ./benchmarks/$folder/python/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"

#   Pypy
pypy ./benchmarks/$folder/python/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"

#   C#
dotnet run --project ./benchmarks/$folder/csharp/Fib.csproj --configuration Release $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"

echo "!!! Finished $testName !!!"
