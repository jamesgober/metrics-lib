window.BENCHMARK_DATA = {
  "lastUpdate": 1757087306139,
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
      }
    ]
  }
}