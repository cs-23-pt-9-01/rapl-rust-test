Body_Count=50000000
count=1 #Testing only #TODO: change to actually useful number

echo "starting N-Body"

#   C
echo - Starting C
gcc -fomit-frame-pointer -march=ivybridge benchmarks/N-Body/benchC/Also_better_than_rust.c -O3 -o benchmarks/N-Body/benchC/Also_better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release
gcc ./benchmarks/N-Body/benchC/Also_better_than_rust $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CNBody"

#   C++
echo - Starting C++
g++ -fomit-frame-pointer -march=ivybridge -std=c++17 benchmarks/N-Body/benchC++/better_than_rust.cpp -O3 -o benchmarks/N-Body/benchC++/better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release
g++ ./benchmarks/N-Body/benchC++/better_than_rust $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CppNBody"

#   Node
echo - Starting Node.js
node ./benchmarks/N-Body/bench.js $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeNBody"

#   Python
echo - Starting Python
python3 ./benchmarks/N-Body/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonNBody"

#   Pypy
echo - Starting PyPy
pypy ./benchmarks/N-Body/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyNBody"

#   C#
echo - Starting C#
dotnet run --project ./benchmarks/N-Body/benchC#/N-Body/N-Body.csproj --configuration Release $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpNBody"

#   Java
echo - Starting Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/N-Body/benchJava/N-Body/src/Bench.java $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaNBody"

