window.BENCHMARK_DATA = {
  "lastUpdate": 1757090676932,
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
      }
    ]
  }
}