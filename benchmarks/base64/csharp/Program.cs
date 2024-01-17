using System;
using System.Runtime.InteropServices;
using System.Text;

// inspired from https://stackoverflow.com/questions/24374658/check-the-operating-system-at-compile-time 
#if _LINUX
    const string pathToLib = @"target/release/librapl_lib.so";
#elif _WINDOWS
    const string pathToLib = @"target\release\rapl_lib.dll";
#else
    const string pathToLib = "none";
#endif

string[] arguments = Environment.GetCommandLineArgs();
uint count = uint.Parse(arguments[1]);

[DllImport(pathToLib)]
static extern int start_rapl();

[DllImport(pathToLib)]
static extern void stop_rapl();

const int STR_SIZE = 131072;
var str1 = Encoding.UTF8.GetBytes(new String('a', STR_SIZE));
var str2 = Convert.ToBase64String(str1);

for (int i = 0; i < count; i++)
{
    start_rapl();

    Convert.ToBase64String(str1);
    Convert.FromBase64String(str2);

    stop_rapl();
}
