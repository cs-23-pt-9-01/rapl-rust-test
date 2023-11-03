runbenchmark(){ #Add optionals for $4 and $5
    language=$1
    testName=$2
    cmd=$3
    input=$4
    inputSize=$5
    echo --- Starting $language ---
    $cmd $input
    sleep 5s
    bash utils/append_to_latest_csv.sh "$language$testName$inputSize"
    echo --- $language Done ---
    echo
}