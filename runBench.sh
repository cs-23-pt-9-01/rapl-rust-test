append_to_lastest_csv () {
    #finding latest csv file
    FILE=$(find . -name "*.csv" | sort -r -t_ | tail -1)
    # append string to name
    timestamp=$(date +%s)
    mv $FILE "${FILE%.csv}_$1_$timestamp.csv"
    echo $FILE
}

# stopping services
bash kill_and_burn.sh 0

echo "starting"

# -- fib --

fibInput=20000
count=1000

#   Node
echo "starting fib"

node ./benchmarks/FibSequence/bench.js $fibInput $count
sleep 5s
append_to_lastest_csv "NodeFib"


#   Pypy
pypy ./benchmarks/FibSequence/bench.py $fibInput $count
sleep 5s
append_to_lastest_csv "PypyFib"

#   C#
# building
dotnet build ./benchmarks/FibSequence/benchC#  # TODO --release

# running
./benchmarks/FibSequence/benchC#/bin/Debug/net7.0/Fib $fibInput $count
sleep 5s
append_to_lastest_csv "CsharpFib" 

# starting services
bash kill_and_burn.sh 1