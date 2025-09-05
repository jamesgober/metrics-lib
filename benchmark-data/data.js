window.BENCHMARK_DATA = {
  "lastUpdate": 1757094108250,
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
      }
    ]
  }
}