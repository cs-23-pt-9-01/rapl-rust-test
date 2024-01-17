from ctypes import *
import sys
import platform
import base64 # for b64encode()
from urllib.request import urlopen

test_count = int(sys.argv[1])
lib_path = "target\\release\\rapl_lib.dll" if platform.system(
) == "Windows" else "target/release/librapl_lib.so"

dll = cdll.LoadLibrary(lib_path)

STR_SIZE = 131072
str1 = b"a" * STR_SIZE
str2 = base64.b64encode(str1)

for i in range(test_count):
    dll.start_rapl()
    base64.b64encode(str1)
    base64.b64decode(str2)
    dll.stop_rapl()
