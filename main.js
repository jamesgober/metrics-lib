(function(){
  function byId(id){return document.getElementById(id)}
  function fmt(v){
    if(typeof v==='number'){return v.toFixed(2)}
    return String(v)
  }
  function friendlyName(name){
    // Remap confusing labels for readability
    if(name === 'tick_n') return 'rate.tick_n';
    if(name === 'tick') return 'rate.tick';
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
    if(['1','2','4','8','16','32','64'].indexOf(String(name))>=0) return 'threads:'+name;
    return name;
  }
  function render(entries){
    if(!entries || !entries.length){ return false }
    var cont = byId('content');
    cont.innerHTML = '';
    var table = document.createElement('table');
    table.style.borderCollapse='collapse';
    table.innerHTML = '<thead><tr><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Name</th><th style="text-align:right;padding:6px;border-bottom:1px solid #e1e4e8">Value</th><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Unit</th></tr></thead>';
    var tbody = document.createElement('tbody');
    // Filter out internal helper rows
    var filtered = entries.filter(function(e){ return e.name !== 'base' && e.name !== 'change' });
    filtered.slice(-100).forEach(function(e){
      var tr = document.createElement('tr');
      tr.innerHTML = '<td style="padding:6px;border-bottom:1px solid #f0f0f0">'+friendlyName(e.name||e.benchmark||'unknown')+'</td>'+
                     '<td style="padding:6px;text-align:right;border-bottom:1px solid #f0f0f0">'+fmt(e.value||e.mean||e.score||e.result||e.time)+'</td>'+
                     '<td style="padding:6px;border-bottom:1px solid #f0f0f0">'+(e.unit||'ns/op')+'</td>';
      tbody.appendChild(tr);
    });
    table.appendChild(tbody);
    cont.appendChild(table);
    // Grouped view with base/change where available
    var section = document.createElement('div');
    section.style.marginTop = '16px';
    section.innerHTML = '<h4>Grouped View</h4>';
    var groups = { rate:[], counter:[], timer:[], gauge:[], threads:[] };
    var pending = null; // capture {base,change}
    entries.forEach(function(e){
      if(e.name==='base' || e.name==='change'){
        pending = pending || {}; pending[e.name] = e; return;
      }
      var name = friendlyName(e.name||e.benchmark||'unknown');
      var bucket = 'other';
      if(name.indexOf('rate.')===0) bucket='rate';
      else if(name.indexOf('counter.')===0) bucket='counter';
      else if(name.indexOf('timer.')===0) bucket='timer';
      else if(name.indexOf('gauge.')===0) bucket='gauge';
      else if(name.indexOf('threads:')===0) bucket='threads';
      if(!groups[bucket]) groups[bucket]=[];
      groups[bucket].push({ item:e, name:name, meta: pending });
      pending = null;
    });
    function addGroup(title, list){
      if(!list || !list.length) return;
      var div=document.createElement('div');
      div.style.marginTop='8px';
      var html = '<div style="font-weight:600;margin:6px 0">'+title+'</div>';
      list.forEach(function(row){
        var v = row.item;
        html += '<div>'+row.name+' &nbsp; <span style="color:#555">'+fmt(v.value)+'</span> <span style="color:#666">'+(v.unit||'ns/op')+'</span></div>';
        if(row.meta && (row.meta.base||row.meta.change)){
          if(row.meta.base){ html += '<div style="margin-left:16px">— base &nbsp; <span style="color:#555">'+fmt(row.meta.base.value)+'</span> <span style="color:#666">'+(row.meta.base.unit||'ns/op')+'</span></div>'; }
          if(row.meta.change){ html += '<div style="margin-left:16px">— change &nbsp; <span style="color:#555">'+fmt(row.meta.change.value)+'</span> <span style="color:#666">'+(row.meta.change.unit||'ns/op')+'</span></div>'; }
        }
      });
      div.innerHTML = html; section.appendChild(div);
    }
    addGroup('Rate', groups.rate);
    addGroup('Counter', groups.counter);
    addGroup('Timer', groups.timer);
    addGroup('Gauge', groups.gauge);
    addGroup('Threads', groups.threads);
    cont.appendChild(section);
    return true;
  }
  function extractJSON(text){
    // Try pure JSON first
    try { return JSON.parse(text) } catch(_){ }
    // Try to find object {...}
    var i = text.indexOf('{'); var j = text.lastIndexOf('}');
    if(i>=0 && j>i){
      var objStr = text.slice(i, j+1);
      try { return JSON.parse(objStr) } catch(_){ }
    }
    // Try to find array [...]
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
    // github-action-benchmark layout: window.BENCHMARK_DATA = { entries: { SuiteName: [ { benches: [...] }, ... ] } }
    if(obj.entries){
      // If entries is an array directly
      if(Array.isArray(obj.entries)) return obj.entries;
      // If entries is an object of suites, pick Criterion or the first suite
      if(typeof obj.entries === 'object'){
        var suite = null;
        if(Array.isArray(obj.entries.Criterion)) suite = obj.entries.Criterion;
        if(!suite){
          var ks = Object.keys(obj.entries);
          for(var i=0;i<ks.length;i++){ if(Array.isArray(obj.entries[ks[i]])){ suite = obj.entries[ks[i]]; break; } }
        }
        if(Array.isArray(suite)){
          // Take the latest run (last element) and return its benches if present
          var last = suite[suite.length-1];
          if(last && Array.isArray(last.benches)) return last.benches;
          return suite;
        }
      }
    }
    if(Array.isArray(obj.bench)) return obj.bench;
    if(obj.data){
      if(Array.isArray(obj.data)) return obj.data;
      if(Array.isArray(obj.data.entries)) return obj.data.entries;
    }
    if(Array.isArray(obj.benchmarks)) return obj.benchmarks;
    return null;
  }
  function buildHistory(obj){
    // Build timeseries per benchmark name across all runs
    if(!obj || !obj.entries || typeof obj.entries!=='object') return null;
    var suite = Array.isArray(obj.entries.Criterion) ? obj.entries.Criterion : null;
    if(!suite){ var ks=Object.keys(obj.entries); for(var i=0;i<ks.length;i++){ if(Array.isArray(obj.entries[ks[i]])){ suite=obj.entries[ks[i]]; break; }}}
    if(!suite) return null;
    var series = {}; var labels = [];
    suite.forEach(function(run, runIdx){
      var ts = (run.date || (run.commit && run.commit.timestamp)) || '';
      labels.push(ts);
      // pad existing series for this run with nulls; will fill real values below
      Object.keys(series).forEach(function(k){ series[k].push(null); });
      var present = {};
      (run.benches||[]).forEach(function(b){
        if(b.name==='base'||b.name==='change') return;
        var name = friendlyName(b.name);
        if(!series[name]){
          // backfill prior runs with nulls
          series[name] = new Array(runIdx).fill(null);
        }
        series[name][runIdx] = b.value;
        present[name] = true;
      });
    });
    return { labels: labels, series: series };
  }
  function loadChartJs(cb){
    if(typeof Chart !== 'undefined'){ cb(); return; }
    var s=document.createElement('script'); s.src='https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js';
    s.onload=function(){ cb(); }; s.onerror=function(){ cb(); }; document.head.appendChild(s);
  }
  function color(idx){ var colors=['#1f77b4','#ff7f0e','#2ca02c','#d62728','#9467bd','#8c564b','#e377c2','#7f7f7f','#bcbd22','#17becf']; return colors[idx%colors.length]; }
  function renderCharts(history){
    if(!history) return;
    function mkChart(el, keys){
      var ctx = byId(el).getContext('2d');
      var datasets = keys.map(function(k, i){ return { label:k, data:history.series[k]||[], borderColor:color(i), backgroundColor:'transparent', tension:0.2, spanGaps:true, pointRadius:2 } });
      new Chart(ctx, { type:'line', data:{ labels: history.labels, datasets: datasets }, options:{ responsive:true, interaction:{ mode:'nearest', intersect:false }, scales:{ y:{ title:{ display:true, text:'ns/op' }}, x:{ ticks:{ maxRotation:45, minRotation:0 } } } } });
    }
    // Choose groups
    var counterKeys = ['counter.inc','counter.add','get'];
    var timerKeys = ['timer.record','timer.record_ns','timer.start_stop','timer.stats'];
    var rateKeys = ['rate.tick','rate.tick_n'];
    var threadKeys = Object.keys(history.series).filter(function(k){ return k.indexOf('threads:')===0; }).sort(function(a,b){ return parseInt(a.split(':')[1]) - parseInt(b.split(':')[1])});
    // Filter to only present keys
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
    var pick = ['counter.inc','counter.add','timer.record','timer.record_ns','rate.tick','rate.tick_n'];
    var map = {}; latest.forEach(function(e){ map[friendlyName(e.name)] = e; });
    pick.forEach(function(k){ if(map[k]){
      var v = map[k]; var card = document.createElement('div'); card.className='card';
      card.innerHTML = '<div class="title">'+k+'</div><div class="value">'+fmt(v.value)+'</div><div class="unit">'+(v.unit||'ns/op')+'</div>';
      el.appendChild(card);
    }});
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
    var onDataDir = /\/benchmark-data(\/?|\/index\.html)$/.test(location.pathname);
    var url = onDataDir ? './data.js' : './benchmark-data/data.js';
    // First try loading as a script to expose any globals the action may set
    var s = document.createElement('script');
    s.src = url + (url.indexOf('?')===-1 ? '?' : '&') + 't=' + Date.now();
    s.onload = function(){
      if(tryKnownGlobals()){
        // If BENCHMARK_DATA exists, build charts and cards
        try{
          var data = window.BENCHMARK_DATA; 
          var latest = deriveEntries(data);
          if(latest) { renderCards(latest); }
          var history = buildHistory(data);
          loadChartJs(function(){ try { if(typeof Chart !== 'undefined') { renderCharts(history); } } catch(_){} });
        }catch(_){ }
        return;
      }
      // Fallback: fetch and attempt to parse as JSON-ish
      fetch(url, { cache: 'no-store' })
        .then(function(r){ if(!r.ok) throw new Error(''+r.status); return r.text() })
        .then(function(txt){
          var parsed = extractJSON(txt);
          var entries = deriveEntries(parsed);
          if(entries){ renderCards(entries); }
          if(!render(entries)) throw new Error('No entries parsed');
          // Try to build history if full object is available
          if(parsed && parsed.entries){ var hist = buildHistory(parsed); loadChartJs(function(){ if(typeof Chart !== 'undefined'){ renderCharts(hist); } }); }
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
