Body_Count=50000000
count=1 #Testing only #TODO: change to actually useful number

echo "starting N-Body"

#   Node
node ./benchmarks/N-Body/bench.js $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeNBody"

#   Python
python3 ./benchmarks/N-Body/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonNBody"

#   Pypy
pypy ./benchmarks/N-Body/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyNBody"

#   C#
dotnet run --project ./benchmarks/N-Body/benchC#/N-Body/N-Body.csproj --configuration Release $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpNBody"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/N-Body/benchJava/N-Body/src/Bench.java $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaNBody"

#   C
gcc -fomit-frame-pointer benchmarks/N-Body/benchC/Also_better_than_rust.c -O3 -o benchmarks/N-Body/benchC/Also_better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/N-Body/benchC/Also_better_than_rust.c $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CNBody"

#   C++
g++ -fomit-frame-pointer -std=c++17 benchmarks/N-Body/benchC++/better_than_rust.cpp -O3 -o benchmarks/N-Body/benchC++/better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/N-Body/benchC++/better_than_rust.cpp $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CppNBody"