from ctypes import *
import sys
import platform
import base64 # for b64encode()
from urllib.request import urlopen

test_count = int(sys.argv[1])
lib_path = "target\\release\\rapl_lib.dll" if platform.system(
) == "Windows" else "target/release/librapl_lib.so"

dll = cdll.LoadLibrary(lib_path)

for i in range(test_count):
    dll.start_rapl()
    print(base64.b64encode(urlopen('http://rosettacode.org/favicon.ico').read()))
    dll.stop_rapl()
