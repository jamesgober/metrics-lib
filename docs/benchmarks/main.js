(function(){
  function byId(id){return document.getElementById(id)}
  function fmt(v){ if(typeof v==='number'){return v.toFixed(2)} return String(v) }
  function friendlyName(name){
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
    var cont = byId('content'); if(!cont) return false;
    cont.innerHTML = '';
    var table = document.createElement('table');
    table.style.borderCollapse='collapse';
    table.innerHTML = '<thead><tr><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Name</th><th style="text-align:right;padding:6px;border-bottom:1px solid #e1e4e8">Value</th><th style="text-align:left;padding:6px;border-bottom:1px solid #e1e4e8">Unit</th></tr></thead>';
    var tbody = document.createElement('tbody');
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
    var rateKeys = ['rate.tick','rate.tick_n'];
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
