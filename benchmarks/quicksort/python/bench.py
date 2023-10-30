# NOTE MUST BE CALLED FROM ROOT

from ctypes import *
import sys
import platform

# used in test method
from heapq import merge

merge_param = sys.argv[1]
# formatting merge_param into a list of integers
merge_param = merge_param.replace("[", "").replace("]", "").split(",")
merge_param = [int(i) for i in merge_param]
test_count =  int(sys.argv[2])
lib_path = "target\\release\\rapl_lib.dll" if platform.system() == "Windows" else "target/release/librapl_lib.so"

# test method
def quickSort(arr):
    less = []
    pivotList = []
    more = []
    if len(arr) <= 1:
        return arr
    else:
        pivot = arr[0]
        for i in arr:
            if i < pivot:
                less.append(i)
            elif i > pivot:
                more.append(i)
            else:
                pivotList.append(i)
        less = quickSort(less)
        more = quickSort(more)
        return less + pivotList + more

# load lib
dll = cdll.LoadLibrary(lib_path)

# running benchmark
for i in range(test_count):
    # start recording
    dll.start_rapl()

    # run test
    result = quickSort(merge_param)

    # stop recording
    dll.stop_rapl()
    print(result)
