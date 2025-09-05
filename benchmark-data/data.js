window.BENCHMARK_DATA = {
  "lastUpdate": 1757098208550,
  "repoUrl": "https://github.com/jamesgober/metrics-lib",
  "entries": {
    "Criterion": [
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "876fcc86254bcf0c46e7e7c27f95f7e11d753ce2",
          "message": "Optimized for v0.9.0",
          "timestamp": "2025-09-05T11:36:19-04:00",
          "tree_id": "085e2f075d02a7635a50b720ed52dd6cf25c8cb6",
          "url": "https://github.com/jamesgober/metrics-lib/commit/876fcc86254bcf0c46e7e7c27f95f7e11d753ce2"
        },
        "date": 1757086910242,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0245717098539884,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128090.00427045109,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124133.18871513393,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36642869908417003,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1172810891581997,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.260206688310344,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141552.7030168936,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45556338092423765,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6200450592148499,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6211535480222824,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6223725353885896,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.38596417294458,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.47663957785835,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.605491910651,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.84714619220165,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.67515574631683,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 47.98588569525859,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.64362794421669,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 358525.4623274161,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47803.532229581295,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 476348.8913460462,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81602.71713264263,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129180.85173189885,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259233.8196834192,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.269662344372016,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3546442713029565,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.897427289711366,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.303405646547546,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 45.99298046528,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0245717098539884,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.018882743463047413,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0245717098539884,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 128090.00427045109,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128090.00427045109,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 124133.18871513393,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007610753447201413,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124133.18871513393,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36642869908417003,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005075783472198503,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36642869908417003,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.1172810891581997,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0603995721193048,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1172810891581997,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.260206688310344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005037535416764105,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.260206688310344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141552.7030168936,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141552.7030168936,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45556338092423765,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002595303303323915,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45556338092423765,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6200450592148499,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0031706830999495006,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6200450592148499,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6211535480222824,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005389152602564118,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6211535480222824,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6223725353885896,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.008330623726247821,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6223725353885896,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.38596417294458,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.007187381078740063,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.38596417294458,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 30.47663957785835,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02229527203245052,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.47663957785835,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.605491910651,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12757448901746626,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.605491910651,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.84714619220165,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00007170882949281143,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.84714619220165,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.67515574631683,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.4324811511220037,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.67515574631683,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47.98588569525859,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39679481024602303,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 47.98588569525859,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.64362794421669,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.3705416129056798,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.64362794421669,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 358525.4623274161,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 358525.4623274161,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47803.532229581295,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004947047810416105,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47803.532229581295,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 476348.8913460462,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.024529374621234057,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 476348.8913460462,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 81602.71713264263,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004864382963512326,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81602.71713264263,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129180.85173189885,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004390537936607375,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129180.85173189885,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259233.8196834192,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004467081460245437,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259233.8196834192,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.269662344372016,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002358445706001411,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.269662344372016,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3546442713029565,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0011147720596929034,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3546442713029565,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.897427289711366,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0030879886544481483,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.897427289711366,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.303405646547546,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001503948245386022,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.303405646547546,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 45.99298046528,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0030341202547570223,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 45.99298046528,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "ab1976a6fa2eb02b7868e6f6b963cdc85a2a0708",
          "message": "Optimized for 0.9.0",
          "timestamp": "2025-09-05T11:42:57-04:00",
          "tree_id": "a2077cda307428268532aa12b44203b48aa7e527",
          "url": "https://github.com/jamesgober/metrics-lib/commit/ab1976a6fa2eb02b7868e6f6b963cdc85a2a0708"
        },
        "date": 1757087305129,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0690415654499494,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131600.97478642213,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122492.76084014161,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3677296123574499,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2918041129886553,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.28539599105644,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 144258.34766868045,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4573767100724253,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220917326393293,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.627510988843028,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.625174663884679,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.45571817953508,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.11929121791231,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.75756164710742,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.98366275802978,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.99094803279942,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.254032011659525,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.03855547985361,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 346279.7345663114,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48831.4873124804,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 486178.3386631757,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83750.55059038238,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130230.38887157681,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257506.50751307927,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.413704607224666,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.381545561750086,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912087222739143,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41277730222499,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.13890144675563,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0690415654499494,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002667563946933349,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0690415654499494,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131600.97478642213,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131600.97478642213,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 122492.76084014161,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005704885860912889,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122492.76084014161,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3677296123574499,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0015435543579847,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3677296123574499,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2918041129886553,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.017049713526242405,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2918041129886553,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.28539599105644,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0002730057648998585,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.28539599105644,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 144258.34766868045,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 144258.34766868045,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4573767100724253,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0013747765687457392,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4573767100724253,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6220917326393293,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00011969723825488998,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220917326393293,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.627510988843028,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004790584150342614,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.627510988843028,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.625174663884679,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0038657978222240708,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.625174663884679,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.45571817953508,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0049808961168072985,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.45571817953508,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.11929121791231,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0016787094579684414,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.11929121791231,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.75756164710742,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12700412622661728,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.75756164710742,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.98366275802978,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011342211833460292,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.98366275802978,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.99094803279942,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.4281807934700712,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.99094803279942,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.254032011659525,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.3934240847228212,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.254032011659525,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.03855547985361,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.3656329904453186,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.03855547985361,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 346279.7345663114,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 346279.7345663114,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48831.4873124804,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01645031954404219,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48831.4873124804,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 486178.3386631757,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004400562954487763,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 486178.3386631757,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83750.55059038238,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021328195523613314,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83750.55059038238,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130230.38887157681,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.012550765696162314,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130230.38887157681,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 257506.50751307927,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.011100460269645174,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257506.50751307927,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.413704607224666,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00008717474181985096,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.413704607224666,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.381545561750086,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00614428996780525,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.381545561750086,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912087222739143,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00010383749581532875,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912087222739143,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41277730222499,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00022119063541214956,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41277730222499,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.13890144675563,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00012893285026982504,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.13890144675563,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "ea26c094516ca8cab6d431d7150b2e6b254b6eda",
          "message": "Cleaned Cargo.toml",
          "timestamp": "2025-09-05T12:15:52-04:00",
          "tree_id": "ff01b940ec17bd73036f4442f0f1e4c4541ade3b",
          "url": "https://github.com/jamesgober/metrics-lib/commit/ea26c094516ca8cab6d431d7150b2e6b254b6eda"
        },
        "date": 1757089275254,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0665206794580846,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132976.65574750205,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 129367.86266029453,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36863049852760404,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.200447417350746,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2867054299912475,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 146131.99553558303,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45749654530250067,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6224609810350384,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.625064369170779,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6262395013053452,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.64833144384262,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.082600397843336,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.829364096949,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.43103197908141,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.906924957222564,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.39156380026342,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.04743560867077,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 333257.0177124653,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50853.15626574421,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 500396.1196959956,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85712.84660186617,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 133326.57053295968,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261597.33148059537,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.47978710973933,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.373051258932848,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.91299726481442,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.44409093018428,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.109829424745996,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0665206794580846,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014459303854532646,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0665206794580846,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132976.65574750205,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132976.65574750205,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129367.86266029453,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05010151528558349,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 129367.86266029453,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36863049852760404,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009025244269509347,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36863049852760404,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.200447417350746,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.023492277214001822,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.200447417350746,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.2867054299912475,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000025326792000845977,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2867054299912475,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 146131.99553558303,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 146131.99553558303,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45749654530250067,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016371423038152866,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45749654530250067,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6224609810350384,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007133276858990456,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6224609810350384,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.625064369170779,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008729787962031832,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.625064369170779,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6262395013053452,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00216911842083356,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6262395013053452,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.64833144384262,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0011119190767110432,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.64833144384262,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.082600397843336,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0028557679772581723,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.082600397843336,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.829364096949,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12298415321439549,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.829364096949,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.43103197908141,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004054774347237666,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.43103197908141,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.906924957222564,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.42932499265245305,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.906924957222564,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.39156380026342,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39169524534765854,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.39156380026342,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.04743560867077,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.365522617792542,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.04743560867077,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 333257.0177124653,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 333257.0177124653,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50853.15626574421,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05853230734849868,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50853.15626574421,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 500396.1196959956,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02471470950136423,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 500396.1196959956,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 85712.84660186617,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.045258166495315555,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85712.84660186617,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133326.57053295968,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.036623803787595666,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 133326.57053295968,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 261597.33148059537,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004609488102782278,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261597.33148059537,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.47978710973933,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009548199216182773,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.47978710973933,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.373051258932848,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00455617848965173,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.373051258932848,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.91299726481442,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00008140912899512465,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.91299726481442,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.44409093018428,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007151061453933849,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.44409093018428,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.109829424745996,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005012462271806184,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.109829424745996,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "5ae2738179f6dd75e17ff743472a5708ba7e1a3e",
          "message": "Updated README",
          "timestamp": "2025-09-05T12:24:48-04:00",
          "tree_id": "75ff05d5581462acaba57785fbeece8f5805daf4",
          "url": "https://github.com/jamesgober/metrics-lib/commit/5ae2738179f6dd75e17ff743472a5708ba7e1a3e"
        },
        "date": 1757089813584,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.088232221439985,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132147.85444716737,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123917.86687127316,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36823049858138485,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2505042592780624,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285375287221507,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142404.07532740373,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45767650042519137,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.62229819749787,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6263044111507888,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6239252107861685,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.61837398836675,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.066230583526327,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.42243188701576,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 98.17892994126149,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93930624681993,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.23469506963914,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.92474390478631,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 344060.1199060494,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50694.92466356666,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478820.94503644924,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82720.86346949912,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131117.38399042227,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259538.0650399385,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.47997662399826,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3751747133929095,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913529749146449,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.431117014352076,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.093554262667276,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.088232221439985,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011967448788970492,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.088232221439985,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132147.85444716737,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132147.85444716737,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123917.86687127316,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005862948467953766,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123917.86687127316,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36823049858138485,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00018355216611598557,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36823049858138485,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2505042592780624,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0012782073230914737,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2505042592780624,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.285375287221507,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0002769218731161205,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285375287221507,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142404.07532740373,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142404.07532740373,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45767650042519137,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0020311337703859955,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45767650042519137,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.62229819749787,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004516250890527207,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.62229819749787,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6263044111507888,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0028585735150499225,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6263044111507888,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6239252107861685,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005856638202270115,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6239252107861685,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.61837398836675,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001642935819732294,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.61837398836675,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.066230583526327,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00338091921031336,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.066230583526327,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.42243188701576,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12826108602490838,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.42243188701576,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 98.17892994126149,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02638821280793968,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 98.17892994126149,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.93930624681993,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.4288840346795699,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93930624681993,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.23469506963914,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39366715919381756,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.23469506963914,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.92474390478631,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.3670475741428657,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.92474390478631,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 344060.1199060494,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 344060.1199060494,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50694.92466356666,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05523864230884645,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50694.92466356666,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 478820.94503644924,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01946708560753807,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478820.94503644924,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82720.86346949912,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00877128119036752,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82720.86346949912,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131117.38399042227,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.019447217396397987,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131117.38399042227,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259538.0650399385,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003298691208935889,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259538.0650399385,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.47997662399826,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009578081989827236,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.47997662399826,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3751747133929095,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004953183691128293,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3751747133929095,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.913529749146449,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00018980073851793122,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913529749146449,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.431117014352076,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005104662274939553,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.431117014352076,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.093554262667276,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0008540344378088482,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.093554262667276,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "4b596a7a1be22d437c4d6104fac72604194042d4",
          "message": "Added Docs",
          "timestamp": "2025-09-05T12:39:14-04:00",
          "tree_id": "d9a3f267db2a8d12f819473e57b726b3ef9763fd",
          "url": "https://github.com/jamesgober/metrics-lib/commit/4b596a7a1be22d437c4d6104fac72604194042d4"
        },
        "date": 1757090676392,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0425316071610267,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128915.41193325506,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121922.03631742235,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3684146879731673,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.265937362332134,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284907153271524,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141324.41067736316,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4576511159615108,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6218365942202172,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239900385813506,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6233045238077064,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.41132604722704,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.55453329602342,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.95451999367344,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.59625578178688,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85472033086863,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.29686726489828,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06432695125,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 364206.4574669288,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48404.617269892224,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 489096.36297451175,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81264.69845165372,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129081.50520995683,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 260246.50540527477,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.432620174434625,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3736685979476935,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.915397536577592,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46570277941796,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.047288322951424,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0425316071610267,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.010179290239927141,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0425316071610267,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 128915.41193325506,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128915.41193325506,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 121922.03631742235,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.010337556400518921,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121922.03631742235,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3684146879731673,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00031655736889013575,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3684146879731673,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.265937362332134,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005570647232586712,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.265937362332134,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284907153271524,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00036546890866617776,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284907153271524,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141324.41067736316,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141324.41067736316,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4576511159615108,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001975557347086321,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4576511159615108,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6218365942202172,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00029048175157608824,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6218365942202172,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6239900385813506,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0008472735654528885,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239900385813506,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233045238077064,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.006845621863653006,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6233045238077064,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.41132604722704,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.006385124736115855,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.41132604722704,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 30.55453329602342,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.019796405438763265,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.55453329602342,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.95451999367344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12251473524728129,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.95451999367344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.59625578178688,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0024162981290207775,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.59625578178688,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.85472033086863,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.43003589844090007,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85472033086863,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.29686726489828,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39288562540955274,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.29686726489828,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06432695125,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.365312672382085,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06432695125,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 364206.4574669288,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 364206.4574669288,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48404.617269892224,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007564819325382333,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48404.617269892224,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 489096.36297451175,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001574987847795084,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 489096.36297451175,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 81264.69845165372,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.008986481350616304,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81264.69845165372,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129081.50520995683,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0036181114873485853,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129081.50520995683,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 260246.50540527477,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005780751050036281,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 260246.50540527477,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.432620174434625,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0002110875717828975,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.432620174434625,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3736685979476935,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004671597400006444,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3736685979476935,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.915397536577592,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005700043872087779,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.915397536577592,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.46570277941796,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001055993747103523,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46570277941796,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.047288322951424,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001856917113044032,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.047288322951424,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "32fcce9fdab97cfc2ee0f9933843fac7997a7be7",
          "message": "Updated Docs for 0.9",
          "timestamp": "2025-09-05T12:43:08-04:00",
          "tree_id": "0ba5842bab54055b773e483f452deb999fdffdfb",
          "url": "https://github.com/jamesgober/metrics-lib/commit/32fcce9fdab97cfc2ee0f9933843fac7997a7be7"
        },
        "date": 1757090916765,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0382921454695047,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134768.72921471993,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126195.45916787577,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36780486619599395,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2666085764780637,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.291464926252791,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 144339.25782995328,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45719811959131673,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6235070274309401,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6268221985762704,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.624112151879051,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.83740272828043,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.72261685047033,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.15425409164106,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.30270083422018,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.883210019970036,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.321204736079636,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.03113232937842,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 347762.9060947235,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49941.265566772716,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 496475.78689934366,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83798.54746257442,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 136506.76733111564,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 270606.753335924,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.415210096991956,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3821710294636596,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912625991516049,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.394928441326854,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.18814266977201,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0382921454695047,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01223375390932202,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0382921454695047,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 134768.72921471993,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134768.72921471993,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 126195.45916787577,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02435056256841639,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126195.45916787577,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36780486619599395,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001339225749059425,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36780486619599395,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2666085764780637,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005868516562240078,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2666085764780637,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.291464926252791,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008749268313186231,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.291464926252791,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 144339.25782995328,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 144339.25782995328,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45719811959131673,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000983772831172347,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45719811959131673,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235070274309401,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0023950275862139048,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6235070274309401,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6268221985762704,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003687671234397616,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6268221985762704,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.624112151879051,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005558771978364518,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.624112151879051,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.83740272828043,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007092693031309771,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.83740272828043,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 30.72261685047033,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.014404207081181508,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.72261685047033,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.15425409164106,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12176559935608333,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.15425409164106,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.30270083422018,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00532739684755934,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.30270083422018,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.883210019970036,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.4296479350302258,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.883210019970036,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.321204736079636,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39257969193121056,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.321204736079636,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.03113232937842,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.36572525406297296,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.03113232937842,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 347762.9060947235,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 347762.9060947235,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49941.265566772716,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03955087460147855,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49941.265566772716,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 496475.78689934366,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01668662430096246,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 496475.78689934366,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83798.54746257442,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021913511781490147,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83798.54746257442,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 136506.76733111564,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.061350065691126554,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 136506.76733111564,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 270606.753335924,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.039208276350953364,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 270606.753335924,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.415210096991956,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00006343604785874479,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.415210096991956,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3821710294636596,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0062612286355820945,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3821710294636596,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912625991516049,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000005833364671747887,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912625991516049,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.394928441326854,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00006034267157328799,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.394928441326854,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.18814266977201,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0011963091917817081,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.18814266977201,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "ff4bc551753fac07399dd3e09edcd1dd273dde4e",
          "message": "Updated API for 0.9.0",
          "timestamp": "2025-09-05T12:47:24-04:00",
          "tree_id": "b63bd92b37ea5e62cfc68909521b5a93f62bc8a1",
          "url": "https://github.com/jamesgober/metrics-lib/commit/ff4bc551753fac07399dd3e09edcd1dd273dde4e"
        },
        "date": 1757091187847,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.067236311354681,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 136877.32258085228,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126838.40112847656,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3677105732414766,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2217007339460655,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2928707540361435,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 146102.1907243106,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.454632804981534,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6218579798033238,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6284169450400414,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.624579027165431,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.0470128062524,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.9120582103813,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.45539203465177,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.59795413108752,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.834056414005005,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.235986854274785,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.177848306368176,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 330655.5912270218,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50729.38740102839,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 495420.5477465793,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85143.89205739813,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 134890.23632535397,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 269305.7255086871,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.45056168353177,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.376674406305397,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911862555509041,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.45371696035889,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.19933600070109,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.067236311354681,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0017927290686816644,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.067236311354681,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 136877.32258085228,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 136877.32258085228,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 126838.40112847656,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.029569434652903404,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126838.40112847656,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3677105732414766,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0015952492104633897,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3677105732414766,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2217007339460655,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.014060546363949267,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2217007339460655,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.2928707540361435,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001140837651745441,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2928707540361435,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 146102.1907243106,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 146102.1907243106,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.454632804981534,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004632694486966682,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.454632804981534,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6218579798033238,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00025610074025628204,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6218579798033238,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6284169450400414,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0062412300714445035,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6284169450400414,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.624579027165431,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004814867166187953,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.624579027165431,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.0470128062524,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013723157824965027,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.0470128062524,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 30.9120582103813,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.008326840421895354,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.9120582103813,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.45539203465177,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.12063613151055086,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.45539203465177,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.59795413108752,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0023994560947996346,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.59795413108752,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.834056414005005,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.43031729300564125,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.834056414005005,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.235986854274785,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39365092085237663,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.235986854274785,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.177848306368176,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.36390169587834487,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.177848306368176,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 330655.5912270218,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 330655.5912270218,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50729.38740102839,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05595600035860615,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50729.38740102839,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 495420.5477465793,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014525698108056817,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 495420.5477465793,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 85143.89205739813,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03831983219016477,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85143.89205739813,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 134890.23632535397,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04878141929577828,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 134890.23632535397,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 269305.7255086871,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.034211952832932546,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 269305.7255086871,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.45056168353177,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004939908521044423,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.45056168353177,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.376674406305397,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005233569212962141,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.376674406305397,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.911862555509041,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00014957037697838071,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911862555509041,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.45371696035889,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008669392580868962,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.45371696035889,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.19933600070109,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014389411957114184,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.19933600070109,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "5b59727cdc00a688b4ba6ad2399b747a22840fa7",
          "message": "Update Guidelines",
          "timestamp": "2025-09-05T13:03:53-04:00",
          "tree_id": "fcefee95e4ae0dbc087c6d3c80913e212344415c",
          "url": "https://github.com/jamesgober/metrics-lib/commit/5b59727cdc00a688b4ba6ad2399b747a22840fa7"
        },
        "date": 1757092151861,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0528644941797314,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 143887.21822950823,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 129142.05716429389,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36849663001519134,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.124124460072774,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.287497747731704,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 153731.77084624898,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45811371517936306,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219733501287039,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6244409368692142,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6251795373903059,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.617681756025476,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.543511153558082,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.80369930931818,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.35920595149433,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93872862997444,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.23963161346221,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.05503821367616,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 328043.49132373894,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48693.97815992833,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 514496.6770164389,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83299.57637269521,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 144218.65134201877,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 280167.48100200726,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.42139670065769,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3723828211725095,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914415463740457,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.416625972367065,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06526450724056,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0528644941797314,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005171923143688861,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0528644941797314,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143887.21822950823,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 143887.21822950823,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129142.05716429389,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.048268612672569544,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 129142.05716429389,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36849663001519134,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005390457333802612,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36849663001519134,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.124124460072774,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.05736264222255849,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.124124460072774,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.287497747731704,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001245392605118134,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.287497747731704,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 153731.77084624898,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 153731.77084624898,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45811371517936306,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0029883662160474778,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45811371517936306,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6219733501287039,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00007062305451388529,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219733501287039,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6244409368692142,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00012528086395424687,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6244409368692142,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6251795373903059,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003858032527307542,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6251795373903059,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.617681756025476,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001423966282949607,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.617681756025476,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 30.543511153558082,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.020150000879400998,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 30.543511153558082,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.80369930931818,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.1230804132608766,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.80369930931818,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.35920595149433,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00476705209489503,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.35920595149433,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.93872862997444,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.4288919004797421,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93872862997444,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.23963161346221,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39360510451231323,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.23963161346221,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.05503821367616,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.37785729619074004,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.05503821367616,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 328043.49132373894,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 328043.49132373894,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48693.97815992833,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013588002016063694,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48693.97815992833,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 514496.6770164389,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0535899304107581,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 514496.6770164389,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83299.57637269521,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0158286175419613,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83299.57637269521,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 144218.65134201877,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.12131052597893444,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 144218.65134201877,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 280167.48100200726,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.07592423852133368,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 280167.48100200726,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42139670065769,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000034114858344702625,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.42139670065769,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3723828211725095,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004431206057838866,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3723828211725095,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.914415463740457,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003700953024097231,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914415463740457,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.416625972367065,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0002818964047481831,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.416625972367065,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.06526450724056,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0014672567300250128,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06526450724056,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "c2f858d405f80c2856c2023a22e19c3845568888",
          "message": "Updated README",
          "timestamp": "2025-09-05T13:11:34-04:00",
          "tree_id": "4914a9d0d33def51b119489b85c972736f592a7a",
          "url": "https://github.com/jamesgober/metrics-lib/commit/c2f858d405f80c2856c2023a22e19c3845568888"
        },
        "date": 1757092640409,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0879050129055012,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129567.22332960069,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123574.2811025024,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681410521349832,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2942382713373206,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283820046680785,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140853.80144838546,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45751955466107735,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6216970200731464,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6242395194730317,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6243241726653734,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.62573455689825,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.13256664467839,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.86974388054804,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.36151176588346,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85353323832711,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.21928502424765,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.60869220279332,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351294.3280543873,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47950.269759666204,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 482986.7684383834,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81319.55172795622,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130750.7230363105,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257560.38605816953,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40281107570902,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3744915973827645,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912109281233838,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40004543320583,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.26646335447712,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0879050129055012,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011808881948336625,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0879050129055012,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129567.22332960069,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129567.22332960069,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123574.2811025024,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0030740028287716825,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123574.2811025024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3681410521349832,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00042641642819418735,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681410521349832,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2942382713373206,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01812993676048702,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2942382713373206,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.283820046680785,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005710939566685447,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283820046680785,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 140853.80144838546,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140853.80144838546,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45751955466107735,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016875187020801707,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45751955466107735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6216970200731464,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005148712529740029,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6216970200731464,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6242395194730317,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0004477968146970879,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6242395194730317,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6243241726653734,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00522094453744415,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6243241726653734,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.62573455689825,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003971258531196842,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.62573455689825,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.13256664467839,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001252827612241103,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.13256664467839,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 235.86974388054804,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.11533137012366523,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.86974388054804,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.36151176588346,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004744186006726325,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.36151176588346,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.85353323832711,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.43005206388481054,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85353323832711,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.21928502424765,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.39386087072419385,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.21928502424765,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.60869220279332,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.37097583526179767,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.60869220279332,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 351294.3280543873,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351294.3280543873,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47950.269759666204,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0018926372742498199,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47950.269759666204,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 482986.7684383834,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.010936283011106385,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 482986.7684383834,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 81319.55172795622,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.008317552044364906,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81319.55172795622,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130750.7230363105,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.016596401753031786,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130750.7230363105,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 257560.38605816953,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.010893551058085094,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257560.38605816953,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40281107570902,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0002589448969040342,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40281107570902,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3744915973827645,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004825466985006077,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3744915973827645,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912109281233838,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00009934730596405927,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912109281233838,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40004543320583,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000020368565615935808,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40004543320583,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.26646335447712,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0028940258767924387,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.26646335447712,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "0d3fa0aec5b38f0968e52c7b4ee019efeff2b923",
          "message": "Updated (MSRV Fix)",
          "timestamp": "2025-09-05T13:20:53-04:00",
          "tree_id": "02fc15c801c1c1931578106591736f3d9122cd11",
          "url": "https://github.com/jamesgober/metrics-lib/commit/0d3fa0aec5b38f0968e52c7b4ee019efeff2b923"
        },
        "date": 1757093223856,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0433481807719773,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131788.87085450714,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124111.96256104924,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3687294375988284,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.246414281398303,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2873549531111745,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142667.03025666712,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.457036096827065,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6221920357370941,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235077959227883,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6233413160530451,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.109416461454366,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.669400472175322,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.83112357894268,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.86719275100785,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.989914911825046,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.284687342393056,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.078433649295015,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331193.5042375413,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47801.121880713195,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 482436.2821073816,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82237.24687666468,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131618.4724871522,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259338.46284530766,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.4123030990803,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372160122780756,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.918020820008066,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40290317056308,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06281365357863,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0433481807719773,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0433481807719773,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131788.87085450714,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131788.87085450714,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 124111.96256104924,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124111.96256104924,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3687294375988284,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3687294375988284,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.246414281398303,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.246414281398303,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.2873549531111745,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2873549531111745,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142667.03025666712,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142667.03025666712,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.457036096827065,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.457036096827065,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6221920357370941,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6221920357370941,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235077959227883,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235077959227883,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233413160530451,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6233413160530451,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.109416461454366,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.109416461454366,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.669400472175322,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.669400472175322,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.83112357894268,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.83112357894268,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.86719275100785,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.86719275100785,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.989914911825046,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.989914911825046,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.284687342393056,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.284687342393056,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.078433649295015,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.078433649295015,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 331193.5042375413,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331193.5042375413,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47801.121880713195,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47801.121880713195,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 482436.2821073816,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 482436.2821073816,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82237.24687666468,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82237.24687666468,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131618.4724871522,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131618.4724871522,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259338.46284530766,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259338.46284530766,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.4123030990803,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.4123030990803,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.372160122780756,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372160122780756,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.918020820008066,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.918020820008066,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40290317056308,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40290317056308,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.06281365357863,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06281365357863,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "89498945cf7d8429678983faa87f8df7e012c73d",
          "message": "Update (MSRV Fix 3)",
          "timestamp": "2025-09-05T13:35:47-04:00",
          "tree_id": "ae3cc808713c3a426ab226e3f3ca7ce68c64d162",
          "url": "https://github.com/jamesgober/metrics-lib/commit/89498945cf7d8429678983faa87f8df7e012c73d"
        },
        "date": 1757094107383,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0654256009645935,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 127112.78876076298,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121855.45993791135,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681110897046349,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.26307585127442,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284056670061989,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 138924.9983848401,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4574931946602399,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220069766363658,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235813978818265,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6228961755245911,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.45758135275285,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.999968700642736,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.8800305733492,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.79268287782352,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88970675719832,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.317108271329964,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.939924264767015,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 329275.23292627756,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48401.89639974402,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 474462.51038123213,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82224.80164004961,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 127577.09256808009,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 254423.7064235752,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40567773416722,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371386935861388,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912580371909735,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41101998221602,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.087304356387904,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0654256009645935,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0654256009645935,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 127112.78876076298,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 127112.78876076298,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 121855.45993791135,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121855.45993791135,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3681110897046349,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681110897046349,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.26307585127442,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.26307585127442,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284056670061989,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284056670061989,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 138924.9983848401,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 138924.9983848401,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4574931946602399,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4574931946602399,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6220069766363658,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220069766363658,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235813978818265,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235813978818265,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6228961755245911,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6228961755245911,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.45758135275285,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.45758135275285,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.999968700642736,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.999968700642736,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.8800305733492,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.8800305733492,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.79268287782352,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.79268287782352,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.88970675719832,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88970675719832,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.317108271329964,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.317108271329964,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.939924264767015,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.939924264767015,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 329275.23292627756,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 329275.23292627756,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48401.89639974402,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48401.89639974402,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 474462.51038123213,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 474462.51038123213,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82224.80164004961,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82224.80164004961,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 127577.09256808009,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 127577.09256808009,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 254423.7064235752,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 254423.7064235752,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40567773416722,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40567773416722,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.371386935861388,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371386935861388,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912580371909735,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912580371909735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41101998221602,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41101998221602,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.087304356387904,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.087304356387904,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "3c8aa03cb79ad8bc3444eb63d3b0d18f9468f0b8",
          "message": "Updated 0.9.0",
          "timestamp": "2025-09-05T13:36:17-04:00",
          "tree_id": "b65ac0a14553b5348fe0777c29af8283200082c0",
          "url": "https://github.com/jamesgober/metrics-lib/commit/3c8aa03cb79ad8bc3444eb63d3b0d18f9468f0b8"
        },
        "date": 1757094143961,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.099744316975561,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132213.04456483375,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124040.64857038927,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682464969735975,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.3021568194585296,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283496586845129,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142997.91507781736,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.46056474406301484,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621807040958242,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6230893010383851,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6232479761457319,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.624703448125157,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.34389371037354,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 231.44829800544704,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 99.15195470759096,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88303842232263,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.301301393096146,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.88163397158448,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 333801.3114346413,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49373.420384384626,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478794.3596722024,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82534.1414534475,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131659.3584250753,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261882.22777508164,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.429157408146835,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.370788049494443,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912268712507353,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41750354385869,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07418376859503,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.099744316975561,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.099744316975561,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132213.04456483375,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132213.04456483375,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 124040.64857038927,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124040.64857038927,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3682464969735975,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682464969735975,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.3021568194585296,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.3021568194585296,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.283496586845129,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283496586845129,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142997.91507781736,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142997.91507781736,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.46056474406301484,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.46056474406301484,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.621807040958242,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621807040958242,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6230893010383851,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6230893010383851,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6232479761457319,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6232479761457319,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.624703448125157,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.624703448125157,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.34389371037354,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.34389371037354,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 231.44829800544704,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 231.44829800544704,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 99.15195470759096,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 99.15195470759096,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.88303842232263,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88303842232263,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.301301393096146,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.301301393096146,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.88163397158448,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.88163397158448,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 333801.3114346413,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 333801.3114346413,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49373.420384384626,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49373.420384384626,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 478794.3596722024,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478794.3596722024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82534.1414534475,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82534.1414534475,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131659.3584250753,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131659.3584250753,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 261882.22777508164,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261882.22777508164,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.429157408146835,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.429157408146835,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.370788049494443,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.370788049494443,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912268712507353,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912268712507353,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41750354385869,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41750354385869,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.07418376859503,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07418376859503,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "ea5d2138376690f9bcfef499654b1fff430822a4",
          "message": "Updated API",
          "timestamp": "2025-09-05T13:44:01-04:00",
          "tree_id": "91b6e0171df8fa93aee5a0746c9ccfd117bc239c",
          "url": "https://github.com/jamesgober/metrics-lib/commit/ea5d2138376690f9bcfef499654b1fff430822a4"
        },
        "date": 1757094601198,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.054557781879361,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134036.89540860275,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124393.19111514471,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682132979937571,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.116928340249811,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.288531143733766,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 145423.90517528,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4559508750576387,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6250010904594494,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6245879953285385,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236543435437543,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.04487749864631,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.717633591563,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.72163662142842,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.79176086297215,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93129742090582,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.309726157196174,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.54244551576504,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351773.5490440851,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50680.621870780764,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 493005.80468565575,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85283.01952310833,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 134421.19028794797,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262761.90456340386,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.717690215460344,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.388638755924534,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.91409891264689,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.419867826394565,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.10879678902517,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.054557781879361,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005261781920470665,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.054557781879361,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 134036.89540860275,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05447214804539868,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134036.89540860275,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 124393.19111514471,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.020825748624857754,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124393.19111514471,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3682132979937571,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000277656098881085,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682132979937571,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.116928340249811,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.06457914830486478,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.116928340249811,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.288531143733766,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008467876011111208,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.288531143733766,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 145423.90517528,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04677996664385131,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 145423.90517528,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4559508750576387,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003371240535603226,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4559508750576387,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6250010904594494,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004813633826544539,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6250010904594494,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6245879953285385,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016142198117696793,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6245879953285385,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6236543435437543,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0012171659563082748,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236543435437543,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.04487749864631,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018669462833386774,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.04487749864631,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.717633591563,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0088229807885426,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.717633591563,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.72163662142842,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0006801525726821689,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.72163662142842,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.79176086297215,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000009147636763295885,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.79176086297215,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.93129742090582,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00099286118063735,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.93129742090582,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.309726157196174,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00015278468430546255,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.309726157196174,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.54244551576504,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011828075123675896,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.54244551576504,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 351773.5490440851,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.06832677914419616,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351773.5490440851,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50680.621870780764,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04707926012272523,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50680.621870780764,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 493005.80468565575,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0390827386752306,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 493005.80468565575,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 85283.01952310833,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03719337501653697,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 85283.01952310833,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 134421.19028794797,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05364676041833771,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 134421.19028794797,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 262761.90456340386,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0327728821226545,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262761.90456340386,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.717690215460344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00492089182614297,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.717690215460344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.388638755924534,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0032117999073881798,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.388638755924534,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.91409891264689,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003091126500112118,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.91409891264689,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.419867826394565,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00013953164893143466,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.419867826394565,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.10879678902517,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00046634171682224945,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.10879678902517,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "d3446adddd12f9e9aceb03dbd2fa5d7211a7f5a5",
          "message": "Added CPU stats example",
          "timestamp": "2025-09-05T13:47:09-04:00",
          "tree_id": "657ab6ed9ef822f80c0f91af38ea61dd62b3d08d",
          "url": "https://github.com/jamesgober/metrics-lib/commit/d3446adddd12f9e9aceb03dbd2fa5d7211a7f5a5"
        },
        "date": 1757094760369,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0847974975585735,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132870.27276312027,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125486.17413876584,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36940861523256024,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.251588707936113,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2873220613570115,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142967.6796750986,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45794714023771876,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6217735002524899,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6246505574014362,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6234544743208931,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.918966224049868,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.462187588398574,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.23725056049835,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.89930412255204,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 42.30277776781542,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.2955271242116,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06922506711908,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 362916.3868790582,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49295.249164268185,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 490471.2884720465,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83539.33595545858,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132085.89281909273,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262047.41970284696,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39888357795749,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.391793970467188,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913385590923228,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.45230785298354,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.08489618703536,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0847974975585735,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009379130666789859,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0847974975585735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132870.27276312027,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.045294293819588605,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132870.27276312027,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125486.17413876584,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.029795252528729055,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125486.17413876584,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36940861523256024,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0035248205343838546,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36940861523256024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.251588707936113,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005075898508588694,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.251588707936113,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.2873220613570115,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006179705288786863,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2873220613570115,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142967.6796750986,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.029099739695945592,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142967.6796750986,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45794714023771876,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009922455301569766,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45794714023771876,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6217735002524899,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0003753597510087747,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6217735002524899,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6246505574014362,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0017145468470376457,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6246505574014362,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6234544743208931,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008962951102273742,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6234544743208931,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.918966224049868,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01466688955273554,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.918966224049868,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.462187588398574,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.016805676195344565,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.462187588398574,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.23725056049835,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005827979255274451,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.23725056049835,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.89930412255204,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001057827231940589,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.89930412255204,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 42.30277776781542,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009860919127731016,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 42.30277776781542,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.2955271242116,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0004466564306201404,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.2955271242116,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06922506711908,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00253829985455023,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06922506711908,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 362916.3868790582,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.10216727706426876,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 362916.3868790582,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49295.249164268185,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01845697856848627,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49295.249164268185,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 490471.2884720465,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03374087043874385,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 490471.2884720465,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83539.33595545858,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01598707797634491,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83539.33595545858,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132085.89281909273,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03534176990752913,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132085.89281909273,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 262047.41970284696,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.029964634139003854,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262047.41970284696,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39888357795749,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00010715375109171532,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39888357795749,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.391793970467188,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0037992114233207364,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.391793970467188,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.913385590923228,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001639095857028927,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913385590923228,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.45230785298354,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006511150708361502,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.45230785298354,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.08489618703536,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00005225233686745323,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.08489618703536,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "b0f79ae9e1506374a53c9cf1781bbbc1063be614",
          "message": "Added memory stats example",
          "timestamp": "2025-09-05T13:47:35-04:00",
          "tree_id": "10959d050f2d9e85b4c8001173acf9bd5fa40f01",
          "url": "https://github.com/jamesgober/metrics-lib/commit/b0f79ae9e1506374a53c9cf1781bbbc1063be614"
        },
        "date": 1757094958463,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.008311792518521,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131789.99083262085,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124229.33646657653,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36689262543486706,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.194332201159049,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.286540516445367,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142229.1114600056,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4549029433106702,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6221343103059742,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6245034118595134,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6241827243532194,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.884774574650528,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.51052478755805,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.86534606817102,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.85210925579176,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85835878829027,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.31566558959331,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.079731180314475,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 334043.6817186998,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49818.01561372287,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 491252.1400157628,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84487.42315291341,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132877.5223419052,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262290.696830783,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.43389067946535,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.37411263565184,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914574340149344,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42520774969061,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.116131824078856,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.008311792518521,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02765231941513613,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.008311792518521,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131789.99083262085,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.036795684505520176,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131789.99083262085,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 124229.33646657653,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.019481084638101054,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 124229.33646657653,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36689262543486706,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0033100449941497745,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36689262543486706,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.194332201159049,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.03037620240464267,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.194332201159049,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.286540516445367,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004700642968973323,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.286540516445367,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142229.1114600056,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02378343072578404,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142229.1114600056,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4549029433106702,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005661835803903892,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4549029433106702,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6221343103059742,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00020471421445611426,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6221343103059742,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6245034118595134,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014785783873905167,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6245034118595134,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6241827243532194,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0020654306113612186,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6241827243532194,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.884774574650528,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01357997670282729,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.884774574650528,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.51052478755805,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01529513724414533,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.51052478755805,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.86534606817102,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00006305609434187343,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.86534606817102,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.85210925579176,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005895901991246344,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.85210925579176,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.85835878829027,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0007483453892325187,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85835878829027,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.31566558959331,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000029858610920086015,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.31566558959331,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.079731180314475,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0027445450217160605,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.079731180314475,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 334043.6817186998,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014481650350817166,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 334043.6817186998,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49818.01561372287,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.029257515083362273,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49818.01561372287,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 491252.1400157628,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03538663069720771,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 491252.1400157628,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 84487.42315291341,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.027517506491152588,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84487.42315291341,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132877.5223419052,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04154687700690918,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132877.5223419052,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 262290.696830783,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.030920823054556346,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262290.696830783,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.43389067946535,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004449592892361576,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.43389067946535,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.37411263565184,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005074480433078232,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.37411263565184,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.914574340149344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00040589020202297377,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914574340149344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42520774969061,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00022374293109561627,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42520774969061,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.116131824078856,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006254969365973029,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.116131824078856,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "8bc88b731d357d71bc9349c56a8f477045553c9d",
          "message": "Added quick tour  example",
          "timestamp": "2025-09-05T13:51:56-04:00",
          "tree_id": "326bed035ce4d89d0b9c66e89f2c2c1315b2a64c",
          "url": "https://github.com/jamesgober/metrics-lib/commit/8bc88b731d357d71bc9349c56a8f477045553c9d"
        },
        "date": 1757095147117,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0298251885159444,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131808.63484867266,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123949.92139548303,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674889102565079,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.12569343949048,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.286454615670681,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142517.36197557702,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45776041461261585,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621822665815278,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6258460900368532,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6240038207063623,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.085004869237906,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.452491454053632,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.34878309101651,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.90867105640474,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85806166235391,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.3085091357196,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.92381584196509,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 319211.49936294794,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48308.91828716339,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478024.91422308254,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81467.94425106866,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130659.17311721494,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259211.8461023109,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.379801181035006,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372991269208223,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911959097545011,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40849880018159,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.15477543954658,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0298251885159444,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.017236356725714552,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0298251885159444,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131808.63484867266,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03694235752114339,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131808.63484867266,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123949.92139548303,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.017188080522931637,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123949.92139548303,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3674889102565079,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0016901947958869457,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674889102565079,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.12569343949048,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.06070605707120902,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.12569343949048,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.286454615670681,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00045380770086711486,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.286454615670681,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142517.36197557702,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.025858294997316733,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142517.36197557702,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45776041461261585,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005840960160607978,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45776041461261585,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.621822665815278,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0002963163244318334,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621822665815278,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6258460900368532,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0036317506627352447,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6258460900368532,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6240038207063623,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001778217984463204,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6240038207063623,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.085004869237906,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01994506537070917,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.085004869237906,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.452491454053632,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01710868068999416,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.452491454053632,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 236.34878309101651,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014895019161270673,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.34878309101651,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.90867105640474,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0011507599090483378,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.90867105640474,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.85806166235391,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0007554384428573035,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85806166235391,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.3085091357196,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00017797289444709818,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.3085091357196,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.92381584196509,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0003162239252301191,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.92381584196509,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 319211.49936294794,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.030563287356576874,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 319211.49936294794,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48308.91828716339,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0019209601171974455,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48308.91828716339,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 478024.91422308254,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007508293624691298,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 478024.91422308254,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 81467.94425106866,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.009204733533979104,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81467.94425106866,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130659.17311721494,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.024158573354304602,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130659.17311721494,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259211.8461023109,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01881955005703828,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259211.8461023109,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.379801181035006,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00040811097770621085,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.379801181035006,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.372991269208223,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00029868139569755314,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372991269208223,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.911959097545011,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00012646599499455125,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911959097545011,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40849880018159,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000039759367301384074,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40849880018159,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.15477543954658,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014639841514039187,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.15477543954658,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "582e9cc5956a40b6a700808bba5c2d168a9d01c1",
          "message": "Added async batch timing example",
          "timestamp": "2025-09-05T13:54:58-04:00",
          "tree_id": "7df3671b7369d6230e17090d736205de8bda5311",
          "url": "https://github.com/jamesgober/metrics-lib/commit/582e9cc5956a40b6a700808bba5c2d168a9d01c1"
        },
        "date": 1757095260862,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 1.9772782886194662,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130420.36761921336,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122761.8825931168,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3668593575536078,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.03703361304189,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285422208914515,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141550.09083402093,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4547451229897014,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219954892470051,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6236211040164358,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236229249694231,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.093373064891495,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.641530060494894,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.22388130038362,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.26736571396069,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.874734361251676,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.314218179941406,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.112138916165556,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 337298.4547142889,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47760.618192663984,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 475582.6870491137,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80715.44562719262,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132378.68729718006,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259055.09224753853,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.69753297777232,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372054571561919,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.916694201736011,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41466787176676,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.10556068907388,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 1.9772782886194662,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.042677553867813356,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 1.9772782886194662,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130420.36761921336,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.026020818917563915,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130420.36761921336,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 122761.8825931168,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007438506700210956,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122761.8825931168,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3668593575536078,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003400419563647117,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3668593575536078,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.03703361304189,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.09988274944705788,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.03703361304189,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.285422208914515,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00025842623154725963,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285422208914515,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141550.09083402093,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01889575295807444,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141550.09083402093,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4547451229897014,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.006006803385522286,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4547451229897014,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6219954892470051,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000018468264492588204,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219954892470051,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6236211040164358,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0000636743410629137,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6236211040164358,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6236229249694231,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0011667264519965581,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236229249694231,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.093373064891495,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.020211080598000475,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.093373064891495,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.641530060494894,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01120121846058686,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.641530060494894,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.22388130038362,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014765144361579274,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.22388130038362,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.26736571396069,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004709496985138895,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.26736571396069,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.874734361251676,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000357424224366798,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.874734361251676,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.314218179941406,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000059815073624180926,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.314218179941406,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.112138916165556,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003380740232424184,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.112138916165556,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 337298.4547142889,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.024366308138965564,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 337298.4547142889,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47760.618192663984,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.013249030612020207,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47760.618192663984,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 475582.6870491137,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002360938205594909,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 475582.6870491137,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 80715.44562719262,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01835645672292907,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80715.44562719262,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132378.68729718006,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03763680949648274,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132378.68729718006,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259055.09224753853,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018203436657167504,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259055.09224753853,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.69753297777232,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004602982793255883,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.69753297777232,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.372054571561919,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00012429484386489165,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372054571561919,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.916694201736011,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008374071292143181,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.916694201736011,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41466787176676,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00005752769079836462,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41466787176676,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.10556068907388,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003961249836788028,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.10556068907388,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "cfd2b7568290311ded5945ddda19e3a75f162233",
          "message": "Added token bucket limiter example",
          "timestamp": "2025-09-05T13:55:30-04:00",
          "tree_id": "3cd330b928a416e9fa18ebcf6d5e8dcb2125e5d8",
          "url": "https://github.com/jamesgober/metrics-lib/commit/cfd2b7568290311ded5945ddda19e3a75f162233"
        },
        "date": 1757095292989,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.076634365408557,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129298.18630766099,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123014.92658486837,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674013220195014,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2609361805971826,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285198436457375,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140942.84677872818,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45783746579784235,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6233483560954586,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239417852896064,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6227927583709866,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.0956496595279,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.397765312524047,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.12090695859624,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.35682418924038,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.90462693685507,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.25485520660831,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.567746140215064,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 352201.145843991,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47625.38360667262,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484479.2597208151,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82229.09004557648,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 128826.24267558055,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262970.64174962585,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40209757685446,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.368076380148668,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911391352581781,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40965616289722,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.070164935729075,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.076634365408557,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005426854609882348,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.076634365408557,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129298.18630766099,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01719258595617079,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129298.18630766099,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123014.92658486837,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009515098031288804,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123014.92658486837,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3674013220195014,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001928134481639865,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674013220195014,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2609361805971826,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000945470155599315,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2609361805971826,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.285198436457375,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00021607762116859774,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285198436457375,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 140942.84677872818,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014524732174539201,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140942.84677872818,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45783746579784235,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007525164125297135,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45783746579784235,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233483560954586,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00215653442722874,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6233483560954586,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6239417852896064,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005779316204814755,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239417852896064,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6227927583709866,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00016602631011086721,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6227927583709866,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.0956496595279,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.020283450899164945,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.0956496595279,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.397765312524047,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01881887428554241,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.397765312524047,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.12090695859624,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003259719662883853,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.12090695859624,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.35682418924038,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004324308830150558,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.35682418924038,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.90462693685507,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00035617770597506926,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.90462693685507,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.25485520660831,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00128842695577025,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.25485520660831,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.567746140215064,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0073062166841376674,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.567746140215064,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 352201.145843991,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.06962537909083011,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 352201.145843991,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47625.38360667262,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01604302415463843,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47625.38360667262,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 484479.2597208151,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02111178253374435,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484479.2597208151,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82229.09004557648,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00005215464727581498,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82229.09004557648,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 128826.24267558055,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009791335437699056,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 128826.24267558055,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 262970.64174962585,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03359331347772021,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 262970.64174962585,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40209757685446,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000056464301631975466,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.40209757685446,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.368076380148668,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000616331638783385,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.368076380148668,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.911391352581781,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00024203559798297025,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911391352581781,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.40965616289722,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000021507607339876778,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.40965616289722,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.070164935729075,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0003718902829787929,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.070164935729075,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "3576256dc05331cdf102da3e62daeba277fd5d2e",
          "message": "Updated memory status example",
          "timestamp": "2025-09-05T14:00:35-04:00",
          "tree_id": "3fa640f722139b0ece089b8d5d6a0e0f7210d900",
          "url": "https://github.com/jamesgober/metrics-lib/commit/3576256dc05331cdf102da3e62daeba277fd5d2e"
        },
        "date": 1757095583628,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 1.9791478561509706,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133139.52416076264,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123425.37231360922,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.367081088433709,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.195212741204227,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283927905895024,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143199.0632046746,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4576359001102797,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6217373235070639,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239113051524098,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.623300143869714,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.13086411746833,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.691005610467904,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 231.7708852551631,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.90253454064869,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85805913473484,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.30188023469432,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.973197905029465,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 332024.55162645027,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48619.41144725099,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483292.77317194047,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81771.15479510975,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131742.38655166744,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258478.67844202626,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39110519137706,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372024141892348,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912406119157624,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.39816806093645,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.15255271845746,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 1.9791478561509706,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.04177238084650914,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 1.9791478561509706,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133139.52416076264,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04741250238276562,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133139.52416076264,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123425.37231360922,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.012883397892041737,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123425.37231360922,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.367081088433709,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002798071831392912,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.367081088433709,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.195212741204227,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.029987112465530874,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.195212741204227,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.283927905895024,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00002436843035658054,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283927905895024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143199.0632046746,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03076526809087876,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143199.0632046746,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4576359001102797,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003119291209254804,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4576359001102797,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6217373235070639,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0004335210687831381,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6217373235070639,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6239113051524098,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005290524568306587,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6239113051524098,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.623300143869714,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006485323894993567,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.623300143869714,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.13086411746833,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021402877645472884,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.13086411746833,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.691005610467904,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.009655106011670078,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.691005610467904,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 231.7708852551631,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004762732620119392,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 231.7708852551631,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.90253454064869,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0010898773570529041,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.90253454064869,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.85805913473484,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0007554987827180781,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.85805913473484,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.30188023469432,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0003151686261960718,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.30188023469432,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.973197905029465,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006531937521050413,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.973197905029465,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 332024.55162645027,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00834960672790186,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 332024.55162645027,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48619.41144725099,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004493936471218873,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48619.41144725099,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 483292.77317194047,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0186110864346547,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483292.77317194047,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 81771.15479510975,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0055171534122486765,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 81771.15479510975,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131742.38655166744,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03264923114127716,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131742.38655166744,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 258478.67844202626,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.015937870238004415,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258478.67844202626,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39110519137706,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00022983025039580163,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39110519137706,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.372024141892348,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001186297018944682,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.372024141892348,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912406119157624,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000035470717814067854,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912406119157624,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39816806093645,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00020267646354177327,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.39816806093645,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.15255271845746,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014157556615808087,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.15255271845746,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "18ca03842bf83778ef439df6868ea3fe9f644c28",
          "message": "Added custom exporter example",
          "timestamp": "2025-09-05T14:02:11-04:00",
          "tree_id": "7635e10f8ced760cc708e60803f543bed1ec53b1",
          "url": "https://github.com/jamesgober/metrics-lib/commit/18ca03842bf83778ef439df6868ea3fe9f644c28"
        },
        "date": 1757095658037,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.064097871007276,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131006.62214551208,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123010.21976049557,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36823114751874436,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2836868920359223,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284156250966734,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141610.9060477226,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4571866301850715,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.623254872690431,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6234429297846237,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6230280542460863,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.14520774397459,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.49803618265667,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.39078023954156,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 99.19694968216987,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87215258569109,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.28478088552501,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.063057361128415,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 340974.11697830213,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48623.153154851265,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483949.06022272765,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82001.2144291747,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130768.57642346833,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258165.28894847128,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39841199146615,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371888074849445,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9120933942304585,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41882941370148,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.12934297744535,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.064097871007276,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0006428360124409727,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.064097871007276,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131006.62214551208,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.030632900298314114,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131006.62214551208,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123010.21976049557,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009476471740967662,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123010.21976049557,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36823114751874436,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00032614560513732904,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36823114751874436,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2836868920359223,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009107534221575175,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2836868920359223,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284156250966734,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000018845540644907288,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284156250966734,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141610.9060477226,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.019333508685328127,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141610.9060477226,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4571866301850715,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0006700962522427911,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4571866301850715,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.623254872690431,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0020062412495973625,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.623254872690431,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6234429297846237,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00022205296321087253,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6234429297846237,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6230280542460863,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00021171862451097034,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6230280542460863,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.14520774397459,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021858844884193873,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.14520774397459,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.49803618265667,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.015685406529037826,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.49803618265667,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.39078023954156,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.006487244365577016,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.39078023954156,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 99.19694968216987,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.015831835705652608,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 99.19694968216987,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.87215258569109,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0004190569203307293,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87215258569109,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.28478088552501,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0006690670646806174,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.28478088552501,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.063057361128415,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0024172218184188576,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.063057361128415,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 340974.11697830213,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.035529195281577275,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 340974.11697830213,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48623.153154851265,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004571241450539887,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48623.153154851265,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 483949.06022272765,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01999430857850726,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483949.06022272765,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82001.2144291747,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0027192186106291816,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82001.2144291747,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130768.57642346833,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.025016119987882224,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130768.57642346833,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 258165.28894847128,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014706108080459135,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258165.28894847128,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39841199146615,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00011459135775715446,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39841199146615,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.371888074849445,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00009329787521195065,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371888074849445,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.9120933942304585,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00009912869457795459,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9120933942304585,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41882941370148,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00012315574623555925,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.41882941370148,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.12934297744535,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009121518744590329,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.12934297744535,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "d2804bc7bd3b99debbe2c169b7be0c514e06b02c",
          "message": "Added axum middleware example",
          "timestamp": "2025-09-05T14:02:58-04:00",
          "tree_id": "621bd00b222ada4a1d42485d025087c01608ff30",
          "url": "https://github.com/jamesgober/metrics-lib/commit/d2804bc7bd3b99debbe2c169b7be0c514e06b02c"
        },
        "date": 1757095751048,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.035179710324601,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131916.647348862,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125264.35322702398,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682423882941466,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.223028671359632,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284211910417394,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143715.30344229873,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4569911395515303,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6224059244786666,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.623929530569781,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6239036510432948,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.00029774914282,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.99584773829312,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.3463024569568,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.03562818591088,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.91156447273036,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.32587657064398,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.025569436388786,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351203.60401538044,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49148.23258358649,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 489721.531380662,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82175.90890843575,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130958.46027114458,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259462.2553509237,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.406348094921626,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371004804276492,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912882183794095,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.47065987933419,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.1939146392198,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.035179710324601,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.014643902266858189,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.035179710324601,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131916.647348862,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03779209499636016,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 131916.647348862,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125264.35322702398,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02797489165318945,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125264.35322702398,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3682423882941466,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00035668197232818777,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682423882941466,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.223028671359632,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.017695907051562543,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.223028671359632,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284211910417394,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00002937901031319079,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284211910417394,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143715.30344229873,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.034481231694449166,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143715.30344229873,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4569911395515303,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0010974045397166288,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4569911395515303,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6224059244786666,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006413880507549763,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6224059244786666,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.623929530569781,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005582794630130117,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.623929530569781,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6239036510432948,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016174052085891066,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6239036510432948,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.00029774914282,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01725232433810353,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.00029774914282,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.99584773829312,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00012878019938611374,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.99584773829312,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.3463024569568,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.006296254255883094,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.3463024569568,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.03562818591088,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00241034667548079,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.03562818591088,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.91156447273036,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005217920397184717,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.91156447273036,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.32587657064398,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00018147400843560035,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.32587657064398,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.025569436388786,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016812975845157396,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.025569436388786,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 351203.60401538044,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.06659587146662949,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351203.60401538044,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49148.23258358649,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0154195649211466,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49148.23258358649,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 489721.531380662,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03216064634309923,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 489721.531380662,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82175.90890843575,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005946226763536844,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82175.90890843575,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130958.46027114458,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.026504505119208988,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130958.46027114458,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259462.2553509237,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01980377142592249,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259462.2553509237,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.406348094921626,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000010572566659039495,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.406348094921626,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.371004804276492,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00007114206990832539,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371004804276492,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912882183794095,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00006143652856782644,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912882183794095,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.47065987933419,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009405289038861842,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.47065987933419,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.1939146392198,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0023132245272470264,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.1939146392198,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "7946d0e17d1224e266d4aeff2121df12a3f10382",
          "message": "Added new examples",
          "timestamp": "2025-09-05T14:03:35-04:00",
          "tree_id": "94c4ea7b5a86a2f645c61a51d281c3061f0dfbdb",
          "url": "https://github.com/jamesgober/metrics-lib/commit/7946d0e17d1224e266d4aeff2121df12a3f10382"
        },
        "date": 1757095785824,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0950190309382006,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133132.03200869792,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125988.65618745338,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36850312148456127,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.293903375521606,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.306244864171966,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 139753.72594011674,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.458244061057991,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.622890820942185,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6247417878836289,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6251208162171323,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.25926770071141,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.73699543887874,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.60073614653132,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.81415314987484,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88915104867276,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.31260716817343,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06620950592294,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331541.0172691277,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50097.88631656982,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483610.1450314245,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84492.71172968543,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131635.52213329877,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259912.4678329299,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39440650809524,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.378152258992899,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9126513492267625,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46966503916416,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.082873618809906,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0950190309382006,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014328005792020093,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0950190309382006,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133132.03200869792,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.047353561404932076,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133132.03200869792,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125988.65618745338,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03391884328899186,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125988.65618745338,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36850312148456127,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0010649822591350944,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36850312148456127,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.293903375521606,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013621958022231428,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.293903375521606,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.306244864171966,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00419908329819596,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.306244864171966,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 139753.72594011674,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005965287492615046,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 139753.72594011674,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.458244061057991,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016412624417478927,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.458244061057991,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.622890820942185,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0014209556146760338,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.622890820942185,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6247417878836289,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0018608476868360313,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6247417878836289,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6251208162171323,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003571447024325769,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6251208162171323,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.25926770071141,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02548467852530578,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.25926770071141,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.73699543887874,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.008217922468115257,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.73699543887874,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.60073614653132,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007388806884582433,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.60073614653132,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.81415314987484,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00021301419347419248,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.81415314987484,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.88915104867276,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000013265992258704173,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88915104867276,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.31260716817343,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00009315754434757739,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.31260716817343,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06620950592294,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002479101470578282,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06620950592294,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 331541.0172691277,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0068811259283427795,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331541.0172691277,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50097.88631656982,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.035039741063426,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50097.88631656982,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 483610.1450314245,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.019279994625586383,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 483610.1450314245,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 84492.71172968543,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.027581825001705695,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84492.71172968543,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131635.52213329877,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03181158532087536,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131635.52213329877,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259912.4678329299,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02157330968293003,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259912.4678329299,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39440650809524,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00017776367156330242,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39440650809524,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.378152258992899,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0012595114096778381,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.378152258992899,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.9126513492267625,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000014448072429118497,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9126513492267625,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.46966503916416,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00092484014552352,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46966503916416,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.082873618809906,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00009613791997331589,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.082873618809906,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "243c3932f321d4c3671d5012abf16fb4a5b7a940",
          "message": "Update 0.9.0",
          "timestamp": "2025-09-05T14:06:44-04:00",
          "tree_id": "402e7ea1459f210025983d55f22c470f6ca61eb7",
          "url": "https://github.com/jamesgober/metrics-lib/commit/243c3932f321d4c3671d5012abf16fb4a5b7a940"
        },
        "date": 1757095964388,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.080434377461558,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134399.52665114554,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125572.77225223853,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681776380947316,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2907723746627653,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.289289454284466,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143529.8095530735,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4583899921605674,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6223790869822287,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6290846315739048,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6250779847464911,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.80187800923162,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 32.259270493368646,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.0556799508651,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.00656646185884,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.9094677511026,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.32509820385326,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.155639814665285,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 356671.7364943081,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50978.84507407252,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 496081.83147059695,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84394.5812896312,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 138093.83188566254,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 265745.5866782192,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.50117241842785,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.376992599643225,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913197746432852,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.50934622698706,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.28543115387398,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.080434377461558,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.007266674960335173,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.080434377461558,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 134399.52665114554,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.05732497855976404,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 134399.52665114554,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125572.77225223853,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.030505915091709968,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125572.77225223853,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3681776380947316,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00018078344271033053,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3681776380947316,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2907723746627653,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.012238442371584002,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2907723746627653,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.289289454284466,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009902967642503313,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.289289454284466,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143529.8095530735,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03314602283080448,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143529.8095530735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4583899921605674,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001960242274190671,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4583899921605674,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6223790869822287,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005982414343246312,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6223790869822287,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6290846315739048,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.008825205034613814,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6290846315739048,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6250779847464911,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0035026852108419604,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6250779847464911,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.80187800923162,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.042733630453160476,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.80187800923162,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.259270493368646,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.008103188948453743,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 32.259270493368646,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.0556799508651,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003539808116885479,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.0556799508651,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.00656646185884,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002122014990855847,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.00656646185884,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.9094677511026,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000471738654529652,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.9094677511026,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.32509820385326,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00016536446010850803,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.32509820385326,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.155639814665285,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004234704959062308,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.155639814665285,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 356671.7364943081,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.08320244229897611,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 356671.7364943081,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50978.84507407252,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.053240655139746185,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50978.84507407252,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 496081.83147059695,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04556592062878395,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 496081.83147059695,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 84394.5812896312,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.026388384116511343,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84394.5812896312,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 138093.83188566254,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0824343861886514,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 138093.83188566254,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 265745.5866782192,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04450009951429146,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 265745.5866782192,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.50117241842785,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001506090427122242,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.50117241842785,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.376992599643225,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0010436157083399955,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.376992599643225,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.913197746432852,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00012567214709546803,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913197746432852,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.50934622698706,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.001550617618177652,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.50934622698706,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.28543115387398,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004298945235633234,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.28543115387398,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "9ea262809b2b80cc036fbf3b374596455e0dc67c",
          "message": "Update 0.9.0",
          "timestamp": "2025-09-05T14:18:22-04:00",
          "tree_id": "15ac2ccc1239407448bc4c17ae3a425762778cd7",
          "url": "https://github.com/jamesgober/metrics-lib/commit/9ea262809b2b80cc036fbf3b374596455e0dc67c"
        },
        "date": 1757096652495,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.039127356485991,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128398.64714887527,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 119079.92176418791,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3704154991416575,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.29466678863726,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284778008162695,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143686.30957087912,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4575962312719482,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220544476669146,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6231374216971656,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6263522659909037,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.98166839761444,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.857565155863032,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.79117007533372,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.78466237839466,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.94812931982774,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.33693277573905,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06975263426344,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 362772.70826757967,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47266.43578922174,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484551.1110795281,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80641.59110554833,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129260.37376987762,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 256463.70836116138,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.45320454188854,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3734410776825685,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912118984579121,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.512742276562165,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.18291057724215,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.039127356485991,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.012732603133378761,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.039127356485991,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 128398.64714887527,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.010115885275181702,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 128398.64714887527,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 119079.92176418791,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02277729840860354,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 119079.92176418791,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3704154991416575,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.006260092405451889,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3704154991416575,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.29466678863726,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013959292325553152,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.29466678863726,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284778008162695,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00013651218102816998,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284778008162695,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143686.30957087912,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.03427253008022069,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143686.30957087912,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4575962312719482,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00022521998777458307,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4575962312719482,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6220544476669146,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00007631912877492653,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220544476669146,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6231374216971656,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000711977916867057,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6231374216971656,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6263522659909037,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0055484213936647375,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6263522659909037,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.98166839761444,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01666011887515073,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.98166839761444,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.857565155863032,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004450115127045184,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.857565155863032,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.79117007533372,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003912484465676602,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.79117007533372,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.78466237839466,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009841780893694141,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.78466237839466,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.94812931982774,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0013946758560057226,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.94812931982774,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.33693277573905,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000410299894144428,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.33693277573905,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06975263426344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00254865650803926,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06975263426344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 362772.70826757967,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.1017309289970254,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 362772.70826757967,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47266.43578922174,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.023459010802896807,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47266.43578922174,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 484551.1110795281,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021263219912126985,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484551.1110795281,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 80641.59110554833,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.019254659213797876,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80641.59110554833,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129260.37376987762,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013194227646309509,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129260.37376987762,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 256463.70836116138,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00801812836650484,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 256463.70836116138,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.45320454188854,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007495670643342667,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.45320454188854,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.3734410776825685,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003824229841769,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.3734410776825685,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912118984579121,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00009391954852333573,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912118984579121,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.512742276562165,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016041737599343797,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.512742276562165,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.18291057724215,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002074458946761837,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.18291057724215,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "0d62ce53b6b629d611f2f9c9b7a988d60c4a4023",
          "message": "Updated (Fixed Stats)",
          "timestamp": "2025-09-05T14:19:02-04:00",
          "tree_id": "c0e70fc3854a1b4655c0393cef41da17e7a86ed5",
          "url": "https://github.com/jamesgober/metrics-lib/commit/0d62ce53b6b629d611f2f9c9b7a988d60c4a4023"
        },
        "date": 1757096688666,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 1.9804535868098612,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132826.2893738306,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122171.13132304803,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3723467456417193,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.200851673444641,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.246903403572335,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141359.23947633558,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4563194858854082,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6178950945893719,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233882187598971,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6235076822562158,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.01073154289005,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.464708615763875,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.36416004317462,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.77341547502888,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.71239432482193,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.03729258988749,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.67313474287582,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 306692.8176696541,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 46988.961178179954,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 471871.5619912517,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 79655.57576926835,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129316.5613975458,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258443.19017961857,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 62.80817465739834,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.319799400531759,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.907074445216964,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.24576191766025,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 45.761038670161376,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 1.9804535868098612,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.04114019605211083,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 1.9804535868098612,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132826.2893738306,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.044948275218954725,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132826.2893738306,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 122171.13132304803,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0025905395236087703,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122171.13132304803,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3723467456417193,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011506461107930743,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3723467456417193,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.200851673444641,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.027495400914086976,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.200851673444641,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.246903403572335,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00703120136847768,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.246903403572335,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141359.23947633558,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.017521980347642963,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141359.23947633558,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4563194858854082,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002565521823124306,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4563194858854082,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6178950945893719,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.006610668692544008,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6178950945893719,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233882187598971,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00030978974450746044,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233882187598971,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235076822562158,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009817153414206636,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6235076822562158,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.01073154289005,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.017584002531357834,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.01073154289005,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.464708615763875,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.016726894013121663,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.464708615763875,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.36416004317462,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.006372935739365593,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.36416004317462,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.77341547502888,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00019115874530295507,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.77341547502888,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.71239432482193,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0042328401438599395,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.71239432482193,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.03729258988749,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005791234025660952,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.03729258988749,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.67313474287582,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005237336445663376,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.67313474287582,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 306692.8176696541,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.06858218596015542,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 306692.8176696541,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46988.961178179954,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02919173269358799,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 46988.961178179954,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 471871.5619912517,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005460807404780166,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 471871.5619912517,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 79655.57576926835,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.03124636143275117,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 79655.57576926835,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129316.5613975458,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013634648622655066,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129316.5613975458,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 258443.19017961857,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.015798385349168553,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258443.19017961857,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 62.80817465739834,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0094234948370705,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 62.80817465739834,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.319799400531759,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00960413687295758,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.319799400531759,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.907074445216964,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0011207809900177779,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.907074445216964,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.24576191766025,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0026061410871819346,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.24576191766025,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 45.761038670161376,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.007079296365514298,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 45.761038670161376,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "0f00d14cecfb0421f07725b9dc5f41383a6ca356",
          "message": "Updated (Fixed Error)",
          "timestamp": "2025-09-05T14:21:06-04:00",
          "tree_id": "6b77ebb5d237b05bddd69fe6eff4f51160783ef1",
          "url": "https://github.com/jamesgober/metrics-lib/commit/0f00d14cecfb0421f07725b9dc5f41383a6ca356"
        },
        "date": 1757096832210,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.027612853765508,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130082.63478090016,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122835.05763757222,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3688047278738415,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.256947790924652,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284882759565713,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143465.97360099189,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4577178157179854,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220495744189912,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233783347681903,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236648514146333,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.306083886266784,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.703229726308095,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.7501769789527,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.06361888175235,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87219501419164,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.395278171190846,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06476754849398,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 320538.2176122909,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48801.16862511131,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 485151.0080083996,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82300.93706264535,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129991.88874705882,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257833.2103971376,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.434908926811566,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371183063745989,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913195504605301,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42390682286243,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.08143246828731,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.027612853765508,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01830748451138897,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.027612853765508,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130082.63478090016,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02336386487221742,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130082.63478090016,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 122835.05763757222,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.008039013599882905,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122835.05763757222,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3688047278738415,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0018843175025320402,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3688047278738415,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.256947790924652,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002707845760590466,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.256947790924652,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284882759565713,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00015633623091226845,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284882759565713,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143465.97360099189,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.032686523440315085,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143465.97360099189,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4577178157179854,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004909822929985985,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4577178157179854,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6220495744189912,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00006848441291729124,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220495744189912,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233783347681903,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00032564010781277464,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233783347681903,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6236648514146333,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0012340353340505317,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6236648514146333,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.306083886266784,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02697291072696162,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.306083886266784,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.703229726308095,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.009273102018024226,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.703229726308095,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 233.7501769789527,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0037364577952916367,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 233.7501769789527,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 101.06361888175235,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0026880523088887998,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 101.06361888175235,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.87219501419164,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00041804405813061774,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87219501419164,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.395278171190846,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016178513710281006,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.395278171190846,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06476754849398,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002450794451088756,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06476754849398,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 320538.2176122909,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.026534079822343792,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 320538.2176122909,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48801.16862511131,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.008249102929144714,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48801.16862511131,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 485151.0080083996,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02252759152367867,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 485151.0080083996,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82300.93706264535,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0009259423078820106,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82300.93706264535,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129991.88874705882,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018928133024274096,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129991.88874705882,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 257833.2103971376,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013400889490565504,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257833.2103971376,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.434908926811566,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00046101853475799537,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.434908926811566,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.371183063745989,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000037955209303053294,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.371183063745989,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.913195504605301,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00012521580289726053,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913195504605301,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42390682286243,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00020322714648068896,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42390682286243,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.08143246828731,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0001274079311557319,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.08143246828731,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "70028c32839a3d58e03acdf8ea54b756872750a5",
          "message": "Updated",
          "timestamp": "2025-09-05T14:27:55-04:00",
          "tree_id": "f875974721bc46a12b238b15ef871ffdc8dacb18",
          "url": "https://github.com/jamesgober/metrics-lib/commit/70028c32839a3d58e03acdf8ea54b756872750a5"
        },
        "date": 1757097235497,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0265461843597037,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130355.6877735067,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123881.5926806795,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3690895599573411,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2579663245380623,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283086618238287,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141244.32520906095,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4571037974861732,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220556231649343,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6227645636017811,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6237254011978064,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.820054258285083,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.62444118223466,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.71359303099015,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.74241552894149,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.976019048335004,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.15480755377817,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.08089068512136,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331424.2231109977,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48204.159502941526,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 488076.77934208966,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82002.00212737995,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129740.03533759047,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 265108.61464776075,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.413175136201936,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022241147330966,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914143763068938,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.396258014675205,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07750803019375,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0265461843597037,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0265461843597037,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130355.6877735067,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130355.6877735067,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123881.5926806795,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123881.5926806795,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3690895599573411,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3690895599573411,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2579663245380623,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2579663245380623,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.283086618238287,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.283086618238287,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141244.32520906095,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141244.32520906095,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4571037974861732,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4571037974861732,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6220556231649343,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6220556231649343,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6227645636017811,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6227645636017811,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6237254011978064,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6237254011978064,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.820054258285083,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.820054258285083,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.62444118223466,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.62444118223466,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 235.71359303099015,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.71359303099015,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.74241552894149,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.74241552894149,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.976019048335004,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.976019048335004,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.15480755377817,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.15480755377817,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.08089068512136,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.08089068512136,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 331424.2231109977,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 331424.2231109977,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48204.159502941526,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48204.159502941526,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 488076.77934208966,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 488076.77934208966,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82002.00212737995,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82002.00212737995,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129740.03533759047,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 129740.03533759047,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 265108.61464776075,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 265108.61464776075,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.413175136201936,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.413175136201936,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.022241147330966,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022241147330966,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.914143763068938,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.914143763068938,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.396258014675205,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.396258014675205,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.07750803019375,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07750803019375,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "00a2d722e2840b7c22373dfc00ea95137d0b9192",
          "message": "Added Broker Throughput Example",
          "timestamp": "2025-09-05T14:28:31-04:00",
          "tree_id": "eec50063916fe09d6e9fe20340e19c604c07d3cb",
          "url": "https://github.com/jamesgober/metrics-lib/commit/00a2d722e2840b7c22373dfc00ea95137d0b9192"
        },
        "date": 1757097313054,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0640013146922898,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 144571.3835465768,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 133519.27254080866,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682040512986727,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1982800955464943,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.28853921858578,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 159766.04447959515,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4581449700256735,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6228390121333457,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6240823263753191,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6254991328420482,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.845644358207945,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.79064110887283,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.62441617877562,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.92381188936203,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.909291983831665,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.21213547125554,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.08158147573879,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351281.2666518616,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 52041.29826761998,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 507012.37771790865,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 92638.99503828933,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 140252.6306492428,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 255856.53885947916,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.43889664727205,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022938560379799,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.919667404126107,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.4538110723797,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.12768439510571,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0640013146922898,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0640013146922898,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 144571.3835465768,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 144571.3835465768,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133519.27254080866,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 133519.27254080866,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3682040512986727,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3682040512986727,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.1982800955464943,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1982800955464943,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.28853921858578,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.28853921858578,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 159766.04447959515,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 159766.04447959515,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4581449700256735,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4581449700256735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6228390121333457,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6228390121333457,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6240823263753191,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6240823263753191,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6254991328420482,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6254991328420482,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.845644358207945,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.845644358207945,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.79064110887283,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.79064110887283,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 236.62441617877562,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.62441617877562,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.92381188936203,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.92381188936203,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.909291983831665,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.909291983831665,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.21213547125554,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.21213547125554,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.08158147573879,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.08158147573879,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 351281.2666518616,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 351281.2666518616,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 52041.29826761998,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 52041.29826761998,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 507012.37771790865,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 507012.37771790865,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 92638.99503828933,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 92638.99503828933,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 140252.6306492428,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 140252.6306492428,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 255856.53885947916,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 255856.53885947916,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.43889664727205,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.43889664727205,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.022938560379799,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022938560379799,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.919667404126107,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.919667404126107,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.4538110723797,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.4538110723797,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.12768439510571,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.12768439510571,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "663cf3324058be321385fced6f39e33a258ca77a",
          "message": "Added cache hit/miss example",
          "timestamp": "2025-09-05T14:29:15-04:00",
          "tree_id": "12a6d704a44af7b80514c38c86567d16d26979c7",
          "url": "https://github.com/jamesgober/metrics-lib/commit/663cf3324058be321385fced6f39e33a258ca77a"
        },
        "date": 1757097363507,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.043743079463055,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133007.3574333672,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125107.24201385818,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36854347106966984,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.264151077206297,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2847316072808415,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143070.4817185663,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45746369826062,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621715441103704,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6230970616277451,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6235947128344903,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.853439118369856,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.924653751036857,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.28131963032524,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.60401011659029,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.86229631636986,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.150980567874576,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 57.725015841630395,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 330937.51240087295,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48821.522314149726,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 487040.9981412794,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83118.54728664724,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131865.9471282815,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259888.63207762697,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.4325003776021,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022117669436712,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9118814598657945,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42130924110535,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.061557889539834,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.043743079463055,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.043743079463055,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133007.3574333672,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133007.3574333672,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125107.24201385818,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125107.24201385818,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36854347106966984,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36854347106966984,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.264151077206297,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.264151077206297,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.2847316072808415,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.2847316072808415,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143070.4817185663,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143070.4817185663,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45746369826062,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45746369826062,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.621715441103704,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621715441103704,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6230970616277451,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6230970616277451,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235947128344903,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6235947128344903,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.853439118369856,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.853439118369856,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.924653751036857,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.924653751036857,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 232.28131963032524,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 232.28131963032524,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.60401011659029,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.60401011659029,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.86229631636986,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.86229631636986,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.150980567874576,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.150980567874576,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 57.725015841630395,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 57.725015841630395,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 330937.51240087295,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 330937.51240087295,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48821.522314149726,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48821.522314149726,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 487040.9981412794,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 487040.9981412794,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83118.54728664724,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83118.54728664724,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131865.9471282815,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131865.9471282815,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 259888.63207762697,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 259888.63207762697,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.4325003776021,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.4325003776021,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.022117669436712,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.022117669436712,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.9118814598657945,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.9118814598657945,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42130924110535,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42130924110535,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.061557889539834,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.061557889539834,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "8437aa44cbe87b3516b228d2d107f9885ba637b3",
          "message": "Added health dashboard example",
          "timestamp": "2025-09-05T14:29:33-04:00",
          "tree_id": "96d9d8ea0a823a7fce589749fc621deaf248de6b",
          "url": "https://github.com/jamesgober/metrics-lib/commit/8437aa44cbe87b3516b228d2d107f9885ba637b3"
        },
        "date": 1757097404544,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0265190441521295,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132523.87071436775,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122453.3575642165,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36746240418322906,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1340966093996085,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.282214238824983,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143677.8322110369,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4566321685890167,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6215312186118762,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233676035212637,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6341659394658933,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.844635033786034,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.78410166784316,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.2407165174895,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.77373131101155,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.84913108178641,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.14440480003616,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06847516014004,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 332665.84031127836,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48224.76251994871,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 491224.4902221386,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82240.61039266556,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 133258.11065649343,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261100.96048073057,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.37114714976652,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.019540404564668,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.909059936231782,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.359119453635024,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06054142406302,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0265190441521295,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0265190441521295,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132523.87071436775,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132523.87071436775,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 122453.3575642165,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 122453.3575642165,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36746240418322906,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36746240418322906,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.1340966093996085,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1340966093996085,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.282214238824983,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.282214238824983,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 143677.8322110369,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 143677.8322110369,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4566321685890167,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4566321685890167,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6215312186118762,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6215312186118762,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233676035212637,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233676035212637,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6341659394658933,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6341659394658933,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.844635033786034,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 31.844635033786034,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.78410166784316,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.78410166784316,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 235.2407165174895,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.2407165174895,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.77373131101155,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.77373131101155,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.84913108178641,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.84913108178641,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.14440480003616,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.14440480003616,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.06847516014004,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.06847516014004,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 332665.84031127836,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 332665.84031127836,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48224.76251994871,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48224.76251994871,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 491224.4902221386,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 491224.4902221386,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82240.61039266556,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82240.61039266556,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133258.11065649343,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 133258.11065649343,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 261100.96048073057,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261100.96048073057,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.37114714976652,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.37114714976652,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.019540404564668,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.019540404564668,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.909059936231782,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.909059936231782,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.359119453635024,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.359119453635024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.06054142406302,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.06054142406302,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "d17b6baaf2dc2de10247988aef3558ca6390986e",
          "message": "Update Fix",
          "timestamp": "2025-09-05T14:34:25-04:00",
          "tree_id": "177d0a3916da6c7443b1c9ce49477b109458a034",
          "url": "https://github.com/jamesgober/metrics-lib/commit/d17b6baaf2dc2de10247988aef3558ca6390986e"
        },
        "date": 1757097629898,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.066539118620515,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132210.90137492362,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125107.38932383129,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36831512300289815,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.254742979139645,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284270443488915,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142008.1199614919,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45797052733347166,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621918488362545,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.624022450295605,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6258735340013344,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.24702565776741,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.6523856108376,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.4727610274726,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.64713914464254,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87233909127327,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.16140457921012,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 57.67186820774093,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 353538.2764185351,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48667.642346061264,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 490068.09391905175,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83198.71327678995,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 135095.9902782802,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258631.6229793735,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.41782370453053,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.021631359407703,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911757292629124,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42766416828069,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.09717885286796,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.066539118620515,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01973452890906957,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.066539118620515,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132210.90137492362,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014231934433427629,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 132210.90137492362,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125107.38932383129,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009894905422401568,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125107.38932383129,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.36831512300289815,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0020982358713491633,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.36831512300289815,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.254742979139645,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0014275436101007788,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.254742979139645,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284270443488915,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00022407833453663706,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284270443488915,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142008.1199614919,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.005407613731032601,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142008.1199614919,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45797052733347166,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0018961335523026701,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45797052733347166,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.621918488362545,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00022045424441563544,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.621918488362545,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.624022450295605,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002019843079299255,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.624022450295605,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6258735340013344,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003444036108522619,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6258735340013344,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.24702565776741,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013418311484216128,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.24702565776741,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.6523856108376,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000883633909668724,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.6523856108376,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 235.4727610274726,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001021714532542517,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 235.4727610274726,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.64713914464254,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0009457425037776579,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.64713914464254,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.87233909127327,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.002469980703561969,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87233909127327,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.16140457921012,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00013699619554263798,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.16140457921012,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 57.67186820774093,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.15157433142208054,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 57.67186820774093,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 353538.2764185351,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.06672431212166163,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 353538.2764185351,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48667.642346061264,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009614996877841175,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 48667.642346061264,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 490068.09391905175,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004079920744531895,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 490068.09391905175,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 83198.71327678995,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014593682085360049,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 83198.71327678995,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 135095.9902782802,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.041282206581439995,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 135095.9902782802,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 258631.6229793735,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02443146435280108,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 258631.6229793735,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.41782370453053,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00007330603330624186,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.41782370453053,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.021631359407703,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00012141749178795624,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.021631359407703,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.911757292629124,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000485633012560327,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911757292629124,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42766416828069,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0004953944379211261,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.42766416828069,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.09717885286796,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00042690725942318153,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.09717885286796,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "01fcdeab3fe630807bad02640bbbb9223afbcb8c",
          "message": "FIxed 0.9.0",
          "timestamp": "2025-09-05T14:36:21-04:00",
          "tree_id": "af3ed45cf92f8d08ea80c231797ad99d6320b8e4",
          "url": "https://github.com/jamesgober/metrics-lib/commit/01fcdeab3fe630807bad02640bbbb9223afbcb8c"
        },
        "date": 1757097731794,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0709206470425756,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133473.18685734642,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126265.46808840153,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3687058348145418,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.264707460311071,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284271229427405,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 145224.55577518814,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45768927730432485,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219567832618101,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6250225354918427,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6237024494907435,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.13019319080036,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.732254340334848,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 230.94191006130444,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.64413909025885,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88079151390523,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.17998809270486,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.66052222535481,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 336178.65067162446,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50276.015400942844,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 500271.33572205086,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84817.60079223516,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 137997.03858705872,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 281317.76653739123,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39095353133598,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.023555861428228,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913224129545994,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.39936917546776,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.11877005899733,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0709206470425756,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021896595806866426,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0709206470425756,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133473.18685734642,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.023915328414793713,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133473.18685734642,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 126265.46808840153,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01924317694128108,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 126265.46808840153,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3687058348145418,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001039653201904911,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3687058348145418,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.264707460311071,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0029854899516217337,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.264707460311071,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284271229427405,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00022422709955716158,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284271229427405,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 145224.55577518814,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.028179755613090274,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 145224.55577518814,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45768927730432485,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0012808465415765635,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45768927730432485,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6219567832618101,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00015889238750277368,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219567832618101,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6250225354918427,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0036257231416678692,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6250225354918427,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6237024494907435,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000036797775140873235,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6237024494907435,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.13019319080036,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00974665002133146,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.13019319080036,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.732254340334848,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0034091719590845226,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.732254340334848,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 230.94191006130444,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.020243562996633657,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 230.94191006130444,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.64413909025885,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0009755219603049259,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.64413909025885,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.88079151390523,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0022686175723363178,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.88079151390523,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.17998809270486,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0005229081000597269,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.17998809270486,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50.66052222535481,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.011573906380336174,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 50.66052222535481,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 336178.65067162446,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.014345443781984768,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 336178.65067162446,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 50276.015400942844,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.04298085309162758,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 50276.015400942844,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 500271.33572205086,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.024984914046513484,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 500271.33572205086,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 84817.60079223516,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.034335730735958414,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 84817.60079223516,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 137997.03858705872,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.06364267766679021,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 137997.03858705872,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 281317.76653739123,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.061141551024914476,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 281317.76653739123,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39095353133598,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00035042567760135146,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.39095353133598,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.023555861428228,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0002617783691969411,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.023555861428228,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.913224129545994,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0001871401341277945,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.913224129545994,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.39936917546776,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00004907483327865059,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.39936917546776,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.11877005899733,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000895491760894318,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.11877005899733,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "ec44bccbc43ee2480995341bebfcc0f99a764984",
          "message": "Updated regression threshold",
          "timestamp": "2025-09-05T14:39:11-04:00",
          "tree_id": "5ec9592d1301cb4a3488ce66111130fb87ba5775",
          "url": "https://github.com/jamesgober/metrics-lib/commit/ec44bccbc43ee2480995341bebfcc0f99a764984"
        },
        "date": 1757097876605,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0450763374786947,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133120.58217600733,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125588.959748886,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3678692354625396,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1818130884588105,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.287152381927567,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142784.4799288276,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45921678787695636,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6230683984726834,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6242170385163436,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6365395443503371,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.244598964037024,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.629110402807164,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.61560201055937,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.72162173995393,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87580649852442,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.404834350202925,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.017848243878504,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 381531.35127839097,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49006.85364709033,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 488506.1957673121,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82959.32917708173,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131466.3487474263,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261204.30657683083,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.458477912536566,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.02114626114985,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.93828173529617,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46057884488392,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.363856723440016,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0450763374786947,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.009143711237375785,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0450763374786947,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 133120.58217600733,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.021210385597478787,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 133120.58217600733,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 125588.959748886,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013782249898961707,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 125588.959748886,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3678692354625396,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003306309977834543,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3678692354625396,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.1818130884588105,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.033726471139834846,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.1818130884588105,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.287152381927567,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007695811148058507,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.287152381927567,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 142784.4799288276,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.010904188309774687,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 142784.4799288276,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.45921678787695636,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004622561445351003,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.45921678787695636,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6230683984726834,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0016281105258659778,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6230683984726834,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6242170385163436,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.002332301803047443,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6242170385163436,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6365395443503371,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.02054452669062745,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6365395443503371,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.244598964037024,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01334204845491116,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.244598964037024,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.629110402807164,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00014764594718363888,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.629110402807164,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 236.61560201055937,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003826716007212294,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 236.61560201055937,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.72162173995393,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00020640550336603614,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.72162173995393,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.87580649852442,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0023873762229618123,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87580649852442,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.404834350202925,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00519214610390728,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.404834350202925,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.017848243878504,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01870888368675727,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.017848243878504,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 381531.35127839097,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.15118728407069937,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 381531.35127839097,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49006.85364709033,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.016651968469646672,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49006.85364709033,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 488506.1957673121,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008798132658580649,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 488506.1957673121,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82959.32917708173,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.01167443507311794,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82959.32917708173,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 131466.3487474263,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.013305942189270059,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 131466.3487474263,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 261204.30657683083,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.014727201815442403,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 261204.30657683083,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.458477912536566,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0007144063711890247,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.458477912536566,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.02114626114985,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00021800748888733956,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.02114626114985,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.93828173529617,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.004911938557564222,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.93828173529617,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.46057884488392,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0010145840184103694,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.46057884488392,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.363856723440016,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.006214500425210323,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.363856723440016,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "5122b56ef8a090226905d0100d9b80965e68d8bb",
          "message": "Added regression check",
          "timestamp": "2025-09-05T14:43:25-04:00",
          "tree_id": "a992a0b0e457c945e60197f6013a827441d19117",
          "url": "https://github.com/jamesgober/metrics-lib/commit/5122b56ef8a090226905d0100d9b80965e68d8bb"
        },
        "date": 1757098127882,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 1.9740719130292377,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129046.2300043337,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121815.19435072331,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674699557429149,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.148812607388049,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285316657440827,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140985.88276608,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.457077697213978,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219254507728316,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235930200629896,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6245929931869069,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.4331801220465,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.744380485160296,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.89528834977315,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.553664392922,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87205029657844,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.170356639494855,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.02426436515415,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 342207.9562243127,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47568.1664346063,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 476423.81734978186,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80979.42528589364,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130448.33848857941,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 263604.24413108756,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.42602711687566,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.020429433641589,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912732318184891,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.394910264286175,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.11745365035011,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 1.9740719130292377,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.025893449522861767,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 1.9740719130292377,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 129046.2300043337,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.010045267617690645,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 129046.2300043337,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 121815.19435072331,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.01668043076651915,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 121815.19435072331,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3674699557429149,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.004388106275922232,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3674699557429149,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.148812607388049,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.048341605436628465,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.148812607388049,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.285316657440827,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00042210915013995987,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.285316657440827,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 140985.88276608,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.001829754523577587,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 140985.88276608,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.457077697213978,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000057099224155865436,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.457077697213978,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6219254507728316,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00020926165965751142,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.6219254507728316,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6235930200629896,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0013302883780301755,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6235930200629896,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6245929931869069,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0013909838968146904,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6245929931869069,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.4331801220465,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.019268536086854082,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.4331801220465,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.744380485160296,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0037926141440567385,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.744380485160296,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.89528834977315,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.003471605819140966,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.89528834977315,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.553664392922,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0018736014520642774,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.553664392922,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.87205029657844,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0024768606960283712,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.87205029657844,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.170356639494855,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.000322897889256879,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.170356639494855,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.02426436515415,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018836998845810582,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.02426436515415,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 342207.9562243127,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.032537552663141955,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 342207.9562243127,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 47568.1664346063,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.013193738359786544,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 47568.1664346063,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 476423.81734978186,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.023875264068115687,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 476423.81734978186,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 80979.42528589364,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.012470144813023776,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 80979.42528589364,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130448.33848857941,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0054594030990196,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 130448.33848857941,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 263604.24413108756,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005674544068181242,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 263604.24413108756,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.42602711687566,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00020267051202083053,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.42602711687566,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.020429433641589,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0003607380920647163,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.020429433641589,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.912732318184891,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00028722091825117424,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.912732318184891,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.394910264286175,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000021259147325647376,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.394910264286175,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.11745365035011,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008669223198916054,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.11745365035011,
            "unit": "ns/op"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "committer": {
            "email": "james.gober@icloud.com",
            "name": "jamesgober",
            "username": "jamesgober"
          },
          "distinct": true,
          "id": "df5ece0c7300d923504a7b8a1a1b3487df2f0184",
          "message": "Update (Fixed MSRV)",
          "timestamp": "2025-09-05T14:44:44-04:00",
          "tree_id": "0605c366e5b7501826d94caee2664cf9529f2991",
          "url": "https://github.com/jamesgober/metrics-lib/commit/df5ece0c7300d923504a7b8a1a1b3487df2f0184"
        },
        "date": 1757098207822,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "add",
            "value": 2.0391906471485752,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130515.9460517057,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123787.2855248194,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3705454912725986,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2250263476941026,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284820256615322,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141240.48229015633,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4578287537759715,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.622221146221384,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233137443325878,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6241181470866181,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.1192693945842,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.74508756083828,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.52925637335696,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.85223596863794,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.83650407409561,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.161272188416014,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.02275331369119,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 366016.54384402663,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49100.685019040575,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484317.6523058117,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82280.45589876873,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132727.7959223567,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257832.01334903,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.399974045754526,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.01890534494428,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911255166622379,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.407715497340995,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07550866399365,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.0391906471485752,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0062394150631541745,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 2.0391906471485752,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 130515.9460517057,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0012293922953132697,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_bursts_4_threads",
            "value": 130515.9460517057,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 123787.2855248194,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000761268513096991,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_increment",
            "value": 123787.2855248194,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.3705454912725986,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003944655913393547,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.3705454912725986,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 2.2250263476941026,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.014588338402566126,
            "unit": "ns/op"
          },
          {
            "name": "increment",
            "value": 2.2250263476941026,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.284820256615322,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0003281487702755381,
            "unit": "ns/op"
          },
          {
            "name": "add",
            "value": 5.284820256615322,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 141240.48229015633,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.000027207598598533167,
            "unit": "ns/op"
          },
          {
            "name": "concurrent_add_set_4_threads",
            "value": 141240.48229015633,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.4578287537759715,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0015859773945987587,
            "unit": "ns/op"
          },
          {
            "name": "get",
            "value": 0.4578287537759715,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.622221146221384,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00026609044317860153,
            "unit": "ns/op"
          },
          {
            "name": "set",
            "value": 0.622221146221384,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6233137443325878,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0008818432565116385,
            "unit": "ns/op"
          },
          {
            "name": "set_max",
            "value": 0.6233137443325878,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 0.6241181470866181,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0006296775601208893,
            "unit": "ns/op"
          },
          {
            "name": "set_min",
            "value": 0.6241181470866181,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 32.1192693945842,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00940335091418687,
            "unit": "ns/op"
          },
          {
            "name": "counter_access",
            "value": 32.1192693945842,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 31.74508756083828,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003814972663339633,
            "unit": "ns/op"
          },
          {
            "name": "gauge_access",
            "value": 31.74508756083828,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 234.52925637335696,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.005024473312735478,
            "unit": "ns/op"
          },
          {
            "name": "mixed_operations",
            "value": 234.52925637335696,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 100.85223596863794,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0010901112418224557,
            "unit": "ns/op"
          },
          {
            "name": "timer_access",
            "value": 100.85223596863794,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 41.83650407409561,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0033236828408798047,
            "unit": "ns/op"
          },
          {
            "name": "rate",
            "value": 41.83650407409561,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 48.161272188416014,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.00013424692084229584,
            "unit": "ns/op"
          },
          {
            "name": "tick",
            "value": 48.161272188416014,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 51.02275331369119,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018806826629576,
            "unit": "ns/op"
          },
          {
            "name": "tick_n",
            "value": 51.02275331369119,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 366016.54384402663,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.1043747509108397,
            "unit": "ns/op"
          },
          {
            "name": "tick_n_concurrent_4_threads",
            "value": 366016.54384402663,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 49100.685019040575,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.018598509451126066,
            "unit": "ns/op"
          },
          {
            "name": "1",
            "value": 49100.685019040575,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 484317.6523058117,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.007701917393704183,
            "unit": "ns/op"
          },
          {
            "name": "16",
            "value": 484317.6523058117,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 82280.45589876873,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.003395694789942283,
            "unit": "ns/op"
          },
          {
            "name": "2",
            "value": 82280.45589876873,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 132727.7959223567,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.023028825119338903,
            "unit": "ns/op"
          },
          {
            "name": "4",
            "value": 132727.7959223567,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 257832.01334903,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.02744762296162606,
            "unit": "ns/op"
          },
          {
            "name": "8",
            "value": 257832.01334903,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.399974045754526,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0002081758312694193,
            "unit": "ns/op"
          },
          {
            "name": "raii_timing",
            "value": 63.399974045754526,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 5.01890534494428,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0006642059369169617,
            "unit": "ns/op"
          },
          {
            "name": "record",
            "value": 5.01890534494428,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 4.911255166622379,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.0005878127677638734,
            "unit": "ns/op"
          },
          {
            "name": "record_ns",
            "value": 4.911255166622379,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 63.407715497340995,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": 0.0001807280591092919,
            "unit": "ns/op"
          },
          {
            "name": "start_stop",
            "value": 63.407715497340995,
            "unit": "ns/op"
          },
          {
            "name": "base",
            "value": 46.07550866399365,
            "unit": "ns/op"
          },
          {
            "name": "change",
            "value": -0.00004339137001041493,
            "unit": "ns/op"
          },
          {
            "name": "stats",
            "value": 46.07550866399365,
            "unit": "ns/op"
          }
        ]
      }
    ]
  }
}