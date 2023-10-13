fibInput=20000
count=1000

echo "starting fib"

#   Node
node ./benchmarks/FibSequence/bench.js $fibInput $count
sleep 5s
#append_to_latest_csv "NodeFib"
bash utils/append_to_latest_csv.sh "NodeFib"

#   Pypy
pypy ./benchmarks/FibSequence/bench.py $fibInput $count
sleep 5s
#append_to_latest_csv "PypyFib"
bash utils/append_to_latest_csv.sh "PypyFib"

#   C#
# building
dotnet build ./benchmarks/FibSequence/benchC#  --configuration Release

# running
./benchmarks/FibSequence/benchC#/bin/Debug/net7.0/Fib $fibInput $count
sleep 5s
#append_to_latest_csv "CsharpFib" 
bash utils/append_to_latest_csv.sh "CsharpFib"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/FibSequence/fibjava/Bench.java $fibInput $count
sleep 5s
#append_to_latest_csv "JavaFib"
bash utils/append_to_latest_csv.sh "JavaFib"
