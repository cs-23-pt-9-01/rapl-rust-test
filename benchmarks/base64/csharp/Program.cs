using System;
using System.Runtime.InteropServices;

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

for (int i = 0; i < count; i++)
{
    start_rapl();

    const string path = "http://rosettacode.org/favicon.ico";

    byte[] input;
    using (var client = new WebClient())
    {
        input = client.DownloadData(path);
    }

    var output = Convert.ToBase64String(input);
    Console.WriteLine(output);

    stop_rapl();
}
