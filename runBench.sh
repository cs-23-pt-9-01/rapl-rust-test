append_to_lastest_csv () {
    #finding latest csv file
    FILE=$(find . -name "*.csv" | sort -r -t_ | tail -1)
    # append string to name
    mv $FILE "${FILE%.csv}_$1.csv"
    echo $FILE
}

# TODO stop services running on the computer

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

# TODO start services again
