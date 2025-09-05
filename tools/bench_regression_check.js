#!/usr/bin/env node
/*
  Bench Regression Check
  - Fetches the GitHub Pages benchmark feed and compares the last two runs
  - Fails (exit 1) if per-benchmark or geomean thresholds are exceeded
*/
const https = require('https');

const FEED_URL = process.env.BENCH_FEED_URL || 'https://jamesgober.github.io/metrics-lib/benchmark-data/data.js';

// Thresholds: same as docs/benchmarks/main.js
const THRESHOLDS = {
  geomean: 0.08, // 8%
  perBenchmark: {
    'counter.inc': 0.08,
    'counter.add': 0.08,
    'timer.record': 0.08,
    'timer.record_ns': 0.08,
    'timer.start_stop': 0.08,
    'rate.tick': 0.08,
    'rate.tick_n': 0.08,
    'rate.tick_n.concurrent(4)': 0.10,
    // noisy sub-ns
    'gauge.set': 0.18,
    'gauge.set_max': 0.18,
    'gauge.set_min': 0.18,
    'gauge.access': 0.18,
  },
};

function get(url){
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if(res.statusCode !== 200){ reject(new Error('HTTP '+res.statusCode)); return; }
      let data='';
      res.on('data', (c)=> data += c);
      res.on('end', ()=> resolve(data));
    }).on('error', reject);
  });
}

function extractJSON(text){
  // Try JSON directly
  try { return JSON.parse(text) } catch(_){ }
  // Try object assignment window.BENCHMARK_DATA = { ... };
  let i = text.indexOf('{'); let j = text.lastIndexOf('}');
  if(i>=0 && j>i){
    const objStr = text.slice(i, j+1);
    try { return JSON.parse(objStr) } catch(_){ }
  }
  // Try array entries
  i = text.indexOf('['); j = text.lastIndexOf(']');
  if(i>=0 && j>i){
    const arrStr = text.slice(i, j+1);
    try { return { entries: { Criterion: JSON.parse(arrStr) } } } catch(_){ }
  }
  return null;
}

function friendlyName(name){
  if(name === 'tick_n') return 'rate.tick_n';
  if(name === 'tick') return 'rate.tick';
  if(name === 'tick_n_concurrent_4_threads') return 'rate.tick_n.concurrent(4)';
  if(name === 'increment') return 'counter.inc';
  if(name === 'add') return 'counter.add';
  if(name === 'get') return 'get';
  if(name === 'set') return 'gauge.set';
  if(name === 'set_max') return 'gauge.set_max';
  if(name === 'set_min') return 'gauge.set_min';
  if(name === 'counter_access') return 'counter.access';
  if(name === 'gauge_access') return 'gauge.access';
  if(name === 'record') return 'timer.record';
  if(name === 'record_ns') return 'timer.record_ns';
  if(name === 'start_stop') return 'timer.start_stop';
  if(name === 'stats') return 'timer.stats';
  return name;
}

function buildHistory(obj){
  if(!obj || !obj.entries || typeof obj.entries!=='object') return null;
  const suite = Array.isArray(obj.entries.Criterion) ? obj.entries.Criterion : null;
  if(!suite){
    const ks = Object.keys(obj.entries);
    for(const k of ks){ if(Array.isArray(obj.entries[k])) return obj.entries[k]; }
    return null;
  }
  return suite;
}

function lastTwo(series){
  if(!series || series.length<2) return null;
  const a = series[series.length-2];
  const b = series[series.length-1];
  if(!(a && b)) return null;
  return { a, b };
}

function geomean(arr){
  const xs = arr.filter((v)=> typeof v==='number' && isFinite(v) && v>0);
  if(!xs.length) return null;
  const sum = xs.reduce((acc,v)=> acc + Math.log(v), 0);
  return Math.exp(sum/xs.length);
}

(async function main(){
  try{
    const txt = await get(FEED_URL);
    const parsed = extractJSON(txt);
    if(!parsed){
      console.error('Could not parse benchmark feed');
      process.exit(0); // do not hard-fail if feed unavailable
    }
    const suite = buildHistory(parsed);
    if(!suite){
      console.error('No suite history available');
      process.exit(0);
    }
    // Build series map: name -> array of values
    const series = {};
    suite.forEach((run)=>{
      (run.benches||[]).forEach((b)=>{
        if(b.name==='base'||b.name==='change') return;
        const n = friendlyName(b.name);
        if(!series[n]) series[n]=[];
        series[n].push(b.value);
      });
    });

    // Per-benchmark checks (based on last two values)
    const breaches = [];
    for(const [name, thr] of Object.entries(THRESHOLDS.perBenchmark)){
      const s = series[name];
      const pair = lastTwo(s);
      if(!pair) continue;
      const {a,b} = pair; if(!(a>0 && b>0)) continue;
      const ratio = (b-a)/a; // positive is regression (slower)
      if(ratio > thr){ breaches.push(`${name}: +${(ratio*100).toFixed(2)}% (> ${(thr*100).toFixed(0)}%)`); }
    }

    // Geomean across core benches
    const coreKeys = ['counter.inc','counter.add','timer.record','timer.record_ns','rate.tick','rate.tick_n'];
    const geos = [];
    coreKeys.forEach((k)=>{
      const s = series[k]; const pair = lastTwo(s); if(!pair) return; const {a,b}=pair; if(a>0&&b>0){ geos.push(b/a); }
    });
    const gm = geomean(geos);
    if(gm!==null){
      const gmr = gm-1; if(gmr > THRESHOLDS.geomean){ breaches.unshift(`geomean: +${(gmr*100).toFixed(2)}% (> ${(THRESHOLDS.geomean*100).toFixed(0)}%)`); }
    }

    if(breaches.length){
      console.error('Benchmark regression detected:\n'+breaches.join('\n'));
      process.exit(1);
    }
    console.log('Benchmark regression check: OK');
  }catch(err){
    // Do not fail the job if the feed is unreachable; log and continue
    console.error('bench_regression_check error:', err.message||err);
    process.exit(0);
  }
})();
