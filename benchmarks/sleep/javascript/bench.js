const os = require("os");

const runCount = process.argv[2];
const sleep_time = process.argv[3];
const libPath = os.platform() == "win32" ?
  "target\\release\\rapl_lib.dll" :
  "target/release/librapl_lib.so"

const koffi = require('koffi');
const lib = koffi.load(libPath);

const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');

for (let i = 0; i < runCount; i++) {
  start();

  const start_time = Date.now();
  let now = start_time;
  while (now - start_time < sleep_time * 1000) {
    now = Date.now();
  }

  stop();
}
