(function(){
  function byId(id){return document.getElementById(id)}
  function fmt(v){ if(typeof v==='number'){return v.toFixed(2)} return String(v) }
  // Alert thresholds (percent deltas e.g., 0.08 == 8%)
  // With >20 runs on the dashboard, tighten to production levels.
  var THRESHOLDS = {
    geomean: 0.08, // 8% geomean guard
    perBenchmark: {
      'counter.inc': 0.08,
      'counter.add': 0.08,
      'timer.record': 0.08,
      'timer.record_ns': 0.08,
      'timer.start_stop': 0.08,
      'rate.tick': 0.08,
      'rate.tick_n': 0.08,
      'rate.tick_n.concurrent(4)': 0.10,
      // Sub-ns gauge operations can be noisier; keep looser
      'gauge.set': 0.18,
      'gauge.set_max': 0.18,
      'gauge.set_min': 0.18,
      'gauge.access': 0.18
    }
  };
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
    if(name === 'concurrent_add_bursts_4_threads') return 'counter.add.concurrent(4)';
    if(name === 'concurrent_add_set_4_threads') return 'gauge.add_set.concurrent(4)';
    if(name === 'counter_access') return 'counter.access';
    if(name === 'gauge_access') return 'gauge.access';
    if(name === 'record') return 'timer.record';
    if(name === 'record_ns') return 'timer.record_ns';
    if(name === 'start_stop') return 'timer.start_stop';
    if(name === 'stats') return 'timer.stats';
    if(['1','2','4','8','16','32','64'].indexOf(String(name))>=0) return 'threads:'+name;
    return name;
  }
  function readCpuMhz(){
    if(window.CPU_MHZ) return window.CPU_MHZ;
    if(window.BENCHMARK_DATA && window.BENCHMARK_DATA.meta && window.BENCHMARK_DATA.meta.cpu_mhz) return window.BENCHMARK_DATA.meta.cpu_mhz;
    var meta = document.querySelector('meta[name="cpu-mhz"]');
    if(meta){ var v = parseFloat(meta.getAttribute('content')); if(!isNaN(v) && v>0) return v; }
    return null;
  }
  function ensureTrendCallout(){
    var host = document.getElementById('summary-cards') || document.body;
    var tn = byId('trend-note');
    if(!tn){ tn = document.createElement('div'); tn.id='trend-note'; tn.className='note'; if(host && host.parentNode){ host.parentNode.insertBefore(tn, host); } else { document.body.insertBefore(tn, document.body.firstChild); } }
    return tn;
  }
  function render(entries){
    if(!entries || !entries.length){ return false }
    var cont = byId('content'); if(!cont) return false;
    cont.innerHTML = '';
    var table = document.createElement('table');
    table.style.borderCollapse='collapse';
    table.innerHTML = '<thead><tr><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Name</th><th style="text-align:right;padding:6px;border-bottom:1px solid #e1e4e8">Value</th><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Unit</th></tr></thead>';
    var tbody = document.createElement('tbody');
    var mhz = readCpuMhz();
    var filtered = entries.filter(function(e){ return e.name !== 'base' && e.name !== 'change' });
    // Optionally augment with derived cycles/op for contention benches
    var augmented = filtered.slice();
    if(mhz){
      var rateConc = filtered.filter(function(e){ return /tick_n_concurrent_/.test(e.name) });
      var others = filtered.filter(function(e){ return /concurrent_/.test(e.name) && !/tick_n_concurrent_/.test(e.name) });
      function pushDerived(arr){
        arr.forEach(function(e){
          if(typeof e.value==='number' || typeof e.mean==='number' || typeof e.time==='number'){
            var val = e.value||e.mean||e.time; // ns/op
            augmented.push({ name: (friendlyName(e.name)||e.name)+'.cycles', value: val * mhz, unit: 'cycles/op' });
          }
        });
      }
      pushDerived(rateConc); pushDerived(others);
    }
    augmented.slice(-120).forEach(function(e){
      var tr = document.createElement('tr');
      tr.innerHTML = '<td style="padding:6px;border-bottom:1px solid #f0f0f0">'+friendlyName(e.name||e.benchmark||'unknown')+'</td>'+
                     '<td style="padding:6px;text-align:right;border-bottom:1px solid #f0f0f0">'+fmt(e.value||e.mean||e.score||e.result||e.time)+'</td>'+
                     '<td style="padding:6px;border-bottom:1px solid #f0f0f0">'+(e.unit||'ns/op')+'</td>';
      tbody.appendChild(tr);
    });
    table.appendChild(tbody);
    cont.appendChild(table);
    return true;
  }
  function extractJSON(text){
    try { return JSON.parse(text) } catch(_){ }
    var i = text.indexOf('{'); var j = text.lastIndexOf('}');
    if(i>=0 && j>i){
      var objStr = text.slice(i, j+1);
      try { return JSON.parse(objStr) } catch(_){ }
    }
    i = text.indexOf('['); j = text.lastIndexOf(']');
    if(i>=0 && j>i){
      var arrStr = text.slice(i, j+1);
      try { return { entries: JSON.parse(arrStr) } } catch(_){ }
    }
    return null;
  }
  function deriveEntries(obj){
    if(!obj) return null;
    if(Array.isArray(obj)) return obj;
    if(obj.entries){
      if(Array.isArray(obj.entries)) return obj.entries;
      if(typeof obj.entries === 'object'){
        var suite = null;
        if(Array.isArray(obj.entries.Criterion)) suite = obj.entries.Criterion;
        if(!suite){ var ks = Object.keys(obj.entries); for(var i=0;i<ks.length;i++){ if(Array.isArray(obj.entries[ks[i]])){ suite = obj.entries[ks[i]]; break; } } }
        if(Array.isArray(suite)){
          var last = suite[suite.length-1];
          if(last && Array.isArray(last.benches)) return last.benches;
          return suite;
        }
      }
    }
    if(Array.isArray(obj.bench)) return obj.bench;
    if(obj.data){ if(Array.isArray(obj.data)) return obj.data; if(Array.isArray(obj.data.entries)) return obj.data.entries; }
    if(Array.isArray(obj.benchmarks)) return obj.benchmarks;
    return null;
  }
  function buildHistory(obj){
    if(!obj || !obj.entries || typeof obj.entries!=='object') return null;
    var suite = Array.isArray(obj.entries.Criterion) ? obj.entries.Criterion : null;
    if(!suite){ var ks=Object.keys(obj.entries); for(var i=0;i<ks.length;i++){ if(Array.isArray(obj.entries[ks[i]])){ suite=obj.entries[ks[i]]; break; }}}
    if(!suite) return null;
    var series = {}; var labels = [];
    suite.forEach(function(run){
      var ts = (run.date || (run.commit && run.commit.timestamp)) || '';
      labels.push(ts);
      (run.benches||[]).forEach(function(b){
        if(b.name==='base'||b.name==='change') return;
        var name = friendlyName(b.name);
        if(!series[name]) series[name]=[];
        series[name].push(b.value);
      });
    });
    return { labels: labels, series: series };
  }
  function color(idx){ var colors=['#1f77b4','#ff7f0e','#2ca02c','#d62728','#9467bd','#8c564b','#e377c2','#7f7f7f','#bcbd22','#17becf']; return colors[idx%colors.length]; }
  function renderCharts(history){
    if(!history) return;
    function mkChart(el, keys){
      if(!document.getElementById(el)) return;
      var ctx = document.getElementById(el).getContext('2d');
      var datasets = keys.map(function(k, i){ return { label:k, data:history.series[k]||[], borderColor:color(i), backgroundColor:'transparent', tension:0.2 } });
      new Chart(ctx, { type:'line', data:{ labels: history.labels, datasets: datasets }, options:{ responsive:true, scales:{ y:{ title:{ display:true, text:'ns/op' }}, x:{ ticks:{ maxRotation:45, minRotation:0 } } } } });
    }
    var counterKeys = ['counter.inc','counter.add','get'];
    var timerKeys = ['timer.record','timer.record_ns','timer.start_stop','timer.stats'];
    var rateKeys = ['rate.tick','rate.tick_n','rate.tick_n.concurrent(4)'];
    var threadKeys = Object.keys(history.series).filter(function(k){ return k.indexOf('threads:')===0; }).sort(function(a,b){ return parseInt(a.split(':')[1]) - parseInt(b.split(':')[1])});
    counterKeys = counterKeys.filter(function(k){return history.series[k]});
    timerKeys = timerKeys.filter(function(k){return history.series[k]});
    rateKeys = rateKeys.filter(function(k){return history.series[k]});
    mkChart('chart-counter', counterKeys);
    mkChart('chart-timer', timerKeys);
    mkChart('chart-rate', rateKeys);
    if(threadKeys.length){ mkChart('chart-threads', threadKeys); }
  }
  function renderCards(latest){
    var el = byId('summary-cards'); if(!el) return;
    el.innerHTML='';
    var pick = ['counter.inc','counter.add','timer.record','timer.record_ns','rate.tick','rate.tick_n','rate.tick_n.concurrent(4)'];
    var map = {}; latest.forEach(function(e){ map[friendlyName(e.name)] = e; });
    pick.forEach(function(k){ if(map[k]){
      var v = map[k]; var card = document.createElement('div'); card.className='card';
      card.innerHTML = '<div class="title">'+k+'</div><div class="value">'+fmt(v.value)+'</div><div class="unit">'+(v.unit||'ns/op')+'</div>';
      el.appendChild(card);
    }});
  }
  function geomean(arr){
    var xs = arr.filter(function(v){ return typeof v==='number' && isFinite(v) && v>0 });
    if(!xs.length) return null;
    var sum = xs.reduce(function(a,b){ return a + Math.log(b) }, 0);
    return Math.exp(sum / xs.length);
  }
  function tryKnownGlobals(){
    var keys = Object.keys(window);
    for(var i=0;i<keys.length;i++){
      var k = keys[i];
      try{
        var v = window[k];
        if(v && typeof v === 'object'){
          var entries = deriveEntries(v) || (v.data && deriveEntries(v.data));
          if(entries && render(entries)) return true;
        }
      }catch(_){ }
    }
    return false;
  }
  function load(){
    var onDataDir = location.pathname.endsWith('/benchmark-data/') || location.pathname.indexOf('/benchmark-data/index.html')>=0;
    var url = onDataDir ? './data.js' : './benchmark-data/data.js';
    var s = document.createElement('script'); s.src = url + (url.indexOf('?')===-1 ? '?' : '&') + 't=' + Date.now();
    s.onload = function(){
      if(tryKnownGlobals()){
        try{
          var data = window.BENCHMARK_DATA; 
          var latest = deriveEntries(data);
          if(latest) { renderCards(latest); }
          var history = buildHistory(data);
          renderCharts(history);
          // Trend note: compare last two points for key series if available
          var tn = ensureTrendCallout();
          if(tn && history && history.series){
            function lastDeltaRatio(key){ var s=history.series[key]; if(!s||s.length<2) return null; var a=s[s.length-2], b=s[s.length-1]; if(!(a>0)) return null; return (b-a)/a; }
            var keys = Object.keys(history.series||{});
            var msgs = [];
            var breaches = [];
            keys.forEach(function(k){
              var d = lastDeltaRatio(k); if(d===null) return;
              var name = k; var thr = THRESHOLDS.perBenchmark[name];
              if(typeof thr==='number'){
                if(d>thr){ breaches.push(name+': +'+fmt(d*100)+'%'); }
              }
            });
            // Geomean across core benches (exclude thread-scaling series)
            var coreKeys = ['counter.inc','counter.add','timer.record','timer.record_ns','rate.tick','rate.tick_n'];
            var geos = [];
            coreKeys.forEach(function(k){ var s=history.series[k]; if(s&&s.length>1){ var a=s[s.length-2], b=s[s.length-1]; if(a>0&&b>0){ geos.push(b/a); } }});
            var gm = geomean(geos);
            if(gm!==null && (gm-1) > THRESHOLDS.geomean){ breaches.unshift('geomean: +'+fmt((gm-1)*100)+'%'); }
            if(breaches.length){ tn.textContent = 'Regression alert — '+breaches.join(' | '); }
            else {
              // Fallback to simple trend note if no breaches
              var simple = [];
              ['rate.tick','rate.tick_n'].forEach(function(k){ var d=lastDeltaRatio(k); if(d!==null){ simple.push(k+': '+(d<0? 'improved ':'regressed ')+fmt(Math.abs(d*100))+'%'); }});
              if(simple.length){ tn.textContent = 'Latest trend — '+simple.join(' | '); }
            }
          }
        }catch(_){ }
        return;
      }
      fetch(url, { cache: 'no-store' })
        .then(function(r){ if(!r.ok) throw new Error(''+r.status); return r.text() })
        .then(function(txt){
          var parsed = extractJSON(txt);
          var entries = deriveEntries(parsed);
          if(entries){ renderCards(entries); }
          if(!render(entries)) throw new Error('No entries parsed');
        })
        .catch(function(){
          var c = byId('content');
          var href = onDataDir ? '.' : './benchmark-data/';
          if(c) c.innerHTML = '<p class="note">No recognizable benchmark data found yet. Browse <a href="'+href+'">raw files</a>.</p>';
        });
    };
    s.onerror = function(){
      var c = byId('content');
      var href = onDataDir ? '.' : './benchmark-data/';
      if(c) c.innerHTML = '<p class="note">No recognizable benchmark data found yet. Browse <a href="'+href+'">raw files</a>.</p>';
    };
    document.head.appendChild(s);
  }
  window.addEventListener('DOMContentLoaded', load);
})();
