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
const TRIES = 8192;

for (let i = 0; i < runCount; i++) {
  start();

  const b = Buffer.from("a".repeat(STR_SIZE));
  const str2 = b.toString('base64');
  const str3 = Buffer.from(str2, 'base64');

  var s_encoded = 0;
  const start = new Date();
  for (var i = 0; i < TRIES; i++) {
    s_encoded += b.toString('base64').length;
  }
  const t_encoded = ((new Date()) - start) / 1000;

  var s_decoded = 0;
  const start1 = new Date();
  for (var i = 0; i < TRIES; i++) {
    s_decoded += Buffer.from(str2, 'base64').length;
  }
  const t_decoded = ((new Date()) - start1) / 1000;

  console.log(util.format("encode %s... to %s...: %d, %d",
    b.toString('utf8', 0, 4),
    str2.substring(0, 4),
    s_encoded, t_encoded));

  console.log(util.format("decode %s... to %s...: %d, %d",
    str2.substring(0, 4),
    str3.toString('utf8', 0, 4),
    s_decoded, t_decoded));

  //var str = "testy";
  //console.log(Buffer.from(str).toString('base64'));

  stop();
}
