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
const int TRIES = 8192;

var str1 = Encoding.UTF8.GetBytes(new String('a', STR_SIZE));
var str2 = Convert.ToBase64String(str1);
var str3 = Convert.FromBase64String(str2);

for (int i = 0; i < count; i++)
{
 
    


    var runtime = Type.GetType("Mono.Runtime") != null ? "Mono" : ".NET Core";

    start_rapl();

    var sw = Stopwatch.StartNew();
    var s_encoded = 0;
    for (var i = 0; i < TRIES; i++)
    {
        s_encoded += Convert.ToBase64String(str1).Length;
    }
    sw.Stop();
    var t_encoded = sw.Elapsed.TotalSeconds;

    var s_decoded = 0;
    sw.Restart();
    for (var i = 0; i < TRIES; i++)
    {
        s_decoded += Convert.FromBase64String(str2).Length;
    }
    sw.Stop();
    var t_decoded = sw.Elapsed.TotalSeconds;

    stop_rapl();

    Console.WriteLine("encode {0}... {1}...: {2}, {3}",
                        Encoding.UTF8.GetString(str1, 0, 4),
                        str2.Substring(0, 4),
                        s_encoded, t_encoded);
    Console.WriteLine("decode {0}... {1}...: {2}, {3}",
                        str2.Substring(0, 4),
                        Encoding.UTF8.GetString(str3, 0, 4),
                        s_decoded, t_decoded);
    Console.WriteLine("overall time: {0}s", t_encoded + t_decoded);
}
