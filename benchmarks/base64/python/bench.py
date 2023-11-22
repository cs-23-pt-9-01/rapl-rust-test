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
    STR_SIZE = 131072
    TRIES = 8192

    str1 = b"a" * STR_SIZE
    str2 = base64.b64encode(str1)
    str3 = base64.b64decode(str2)

    t, s_encoded = time.time(), 0
    for _ in range(0, TRIES):
        s_encoded += len(base64.b64encode(str1))
    t_encoded = time.time() - t

    t, s_decoded = time.time(), 0
    for _ in range(0, TRIES):
        s_decoded += len(base64.b64decode(str2))
    t_decoded = time.time() - t

    print(
        "encode {0}... to {1}...: {2}, {3}".format(
            str1[:4], str2[:4], s_encoded, t_encoded
        )
    )
    print(
        "decode {0}... to {1}...: {2}, {3} ".format(
            str2[:4], str3[:4], s_decoded, t_decoded
        )
    )
    dll.stop_rapl()
