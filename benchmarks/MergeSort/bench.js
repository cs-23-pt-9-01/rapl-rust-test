const os = require("os");

let data = process.argv[2];
data = data.replace("[", "").replace("]", "").split(",");
const mergeParam = data
const runCount = process.argv[3];
const libPath = os.platform() == "win32"?
  "target\\release\\rapl_lib.dll":
  "target/release/librapl_lib.so"

// test method
function mergeSortInPlaceFast(v) {
  sort(v, 0, v.length, v.slice());

  function sort(v, lo, hi, t) {
      let n = hi - lo;
      if (n <= 1) {
          return;
      }
      let mid = lo + Math.floor(n / 2);
      sort(v, lo, mid, t);
      sort(v, mid, hi, t);
      for (let i = lo; i < hi; i++) {
          t[i] = v[i];
      }
      let i = lo, j = mid;
      for (let k = lo; k < hi; k++) {
          if (i < mid && (j >= hi || t[i] < t[j])) {
              v[k] = t[i++];
          } else {
              v[k] = t[j++];
          }
      }
  }
}

const koffi = require('koffi');
const lib = koffi.load(libPath);

const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');


for (let i = 0; i < runCount; i++){

    let tobeSorted = Array.from(mergeParam);
    start();

    mergeSortInPlaceFast(tobeSorted);

    stop();
    console.log(tobeSorted);
}

console.log("js job done");  
