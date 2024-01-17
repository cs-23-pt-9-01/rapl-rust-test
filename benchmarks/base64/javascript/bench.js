const os = require("os");

const runCount = process.argv[2];
const libPath = os.platform() == "win32" ?
  "target\\release\\rapl_lib.dll" :
  "target/release/librapl_lib.so"

const koffi = require('koffi');
const lib = koffi.load(libPath);

const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');

const STR_SIZE = 131072;

for (let i = 0; i < runCount; i++) {
  const b = Buffer.from("a".repeat(STR_SIZE));
  const str2 = b.toString('base64');

  start();

  b.toString('base64');
  Buffer.from(str2, 'base64').length;

  stop();
}
