append_to_lastest_csv () {
    #finding latest csv file
    FILE=$(find . -name "*.csv" | sort -t_ | tail -1)
    # append string to name
    mv $FILE "${FILE%.csv}_$1.csv"
    echo $FILE
}

# TODO stop services running on the computer

echo "starting"

# -- fib --
#   Node
echo "starting fib"

node ./benchmarks/FibSequence/bench.js
append_to_lastest_csv "NodeFib"

#   Pypy
pypy ./benchmarks/FibSequence/bench.py
append_to_lastest_csv "PypyFib"

#   C#
# building
cd benchmarks/FibSequence/benchC#
dotnet build # todo --release
cd ../../..

# running
./benchmarks/FibSequence/benchC#/bin/Debug/net7.0/Fib
append_to_lastest_csv "CsharpFib"

# TODO start services again