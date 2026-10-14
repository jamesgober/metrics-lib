//! Strict validators for the Prometheus 0.0.4 and OpenMetrics 1.0 text
//! formats, written from the specifications, used to check every exporter
//! output in the integration tests.
//!
//! They check grammar (names, labels, escapes, numbers), metadata rules (one
//! TYPE/HELP/UNIT per family, before its samples, UNIT a suffix of the name),
//! family rules (sample names allowed for the type, no interleaving, no name
//! clashes, no duplicate series) and value rules (counters non-negative,
//! histograms cumulative with `+Inf` equal to `_count`, OpenMetrics `_sum`
//! omitted with negative buckets, exemplar placement and size, `# EOF`).

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used)]
// One validator per format, written as a single pass that mirrors the spec.
#![allow(clippy::too_many_lines, clippy::assigning_clones)]

use std::collections::{BTreeMap, HashMap, HashSet};

type Labels = Vec<(String, String)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Prometheus,
    OpenMetrics,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub name: String,
    pub labels: Vec<(String, String)>,
    pub value: f64,
    pub raw_value: String,
    pub exemplar: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Family {
    pub name: String,
    pub ty: String,
    pub help: Option<String>,
    pub unit: Option<String>,
    pub samples: Vec<Sample>,
}

/// A parsed, validated exposition.
#[derive(Debug, Default)]
pub struct Exposition {
    pub families: Vec<Family>,
}

impl Exposition {
    pub fn family(&self, name: &str) -> Option<&Family> {
        self.families.iter().find(|f| f.name == name)
    }

    pub fn value(&self, sample_name: &str, labels: &[(&str, &str)]) -> Option<f64> {
        let mut want: Vec<(String, String)> = labels
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        want.sort();
        self.families
            .iter()
            .flat_map(|f| f.samples.iter())
            .find(|s| {
                let mut have = s.labels.clone();
                have.sort();
                s.name == sample_name && have == want
            })
            .map(|s| s.value)
    }
}

fn is_metric_name(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_alphabetic() || b[0] == b'_' || b[0] == b':')
        && b.iter()
            .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b':')
}

fn is_label_name(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_alphabetic() || b[0] == b'_')
        && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_')
}

fn parse_number(s: &str) -> Result<f64, String> {
    match s {
        "NaN" => Ok(f64::NAN),
        "+Inf" | "Inf" => Ok(f64::INFINITY),
        "-Inf" => Ok(f64::NEG_INFINITY),
        _ => {
            if s.is_empty()
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_digit() || matches!(b, b'.' | b'e' | b'E' | b'+' | b'-'))
            {
                return Err(format!("invalid number {s:?}"));
            }
            s.parse::<f64>()
                .map_err(|e| format!("invalid number {s:?}: {e}"))
        }
    }
}

/// Parse `{k="v",...}` starting at `rest`; returns labels and the remainder.
fn parse_labels(rest: &str) -> Result<(Labels, &str), String> {
    let mut labels = Vec::new();
    let mut chars = rest.char_indices().peekable();
    // rest starts with '{'
    let _ = chars.next();
    loop {
        // Skip to label name or '}'.
        let (start, c) = *chars.peek().ok_or("unterminated label set")?;
        if c == '}' {
            let _ = chars.next();
            return Ok((labels, &rest[start + 1..]));
        }
        let mut end = start;
        while let Some(&(i, c)) = chars.peek() {
            if c == '=' {
                end = i;
                break;
            }
            let _ = chars.next();
        }
        let name = &rest[start..end];
        if !is_label_name(name) {
            return Err(format!("invalid label name {name:?}"));
        }
        let _ = chars.next(); // '='
        match chars.next() {
            Some((_, '"')) => {}
            other => return Err(format!("expected quote, got {other:?}")),
        }
        let mut value = String::new();
        loop {
            match chars.next().ok_or("unterminated label value")? {
                (_, '\\') => match chars.next().ok_or("dangling escape")?.1 {
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    'n' => value.push('\n'),
                    other => return Err(format!("invalid escape \\{other}")),
                },
                (_, '"') => break,
                (_, '\n') => return Err("raw newline in label value".into()),
                (_, c) => value.push(c),
            }
        }
        if labels.iter().any(|(k, _): &(String, String)| k == name) {
            return Err(format!("duplicate label {name}"));
        }
        labels.push((name.to_owned(), value));
        match chars.peek() {
            Some((_, ',')) => {
                let _ = chars.next();
            }
            Some((_, '}')) => {}
            other => return Err(format!("expected , or }} after label, got {other:?}")),
        }
    }
}

fn parse_sample(line: &str, format: Format) -> Result<Sample, String> {
    let name_end = line
        .find(['{', ' '])
        .ok_or_else(|| format!("no value in {line:?}"))?;
    let name = &line[..name_end];
    if !is_metric_name(name) {
        return Err(format!("invalid metric name {name:?}"));
    }
    let (labels, rest) = if line[name_end..].starts_with('{') {
        parse_labels(&line[name_end..])?
    } else {
        (Vec::new(), &line[name_end..])
    };
    let rest = rest
        .strip_prefix(' ')
        .ok_or_else(|| format!("missing space before value in {line:?}"))?;
    let (value_part, exemplar) = match rest.split_once(" # ") {
        Some((v, e)) if format == Format::OpenMetrics => (v, Some(e.to_owned())),
        Some(_) => return Err(format!("exemplar in Prometheus format: {line:?}")),
        None => (rest, None),
    };
    let mut fields = value_part.split(' ');
    let raw_value = fields.next().unwrap_or_default().to_owned();
    let value = parse_number(&raw_value)?;
    if let Some(ts) = fields.next() {
        let _ = parse_number(ts)?;
    }
    if fields.next().is_some() {
        return Err(format!("trailing fields in {line:?}"));
    }
    Ok(Sample {
        name: name.to_owned(),
        labels,
        value,
        raw_value,
        exemplar,
    })
}

fn allowed_suffixes(ty: &str, format: Format) -> &'static [&'static str] {
    match (ty, format) {
        ("counter", Format::OpenMetrics) => &["_total", "_created"],
        ("counter" | "gauge" | "untyped" | "unknown", _) => &[""],
        ("summary", _) => &["", "_sum", "_count", "_created"],
        ("histogram", _) => &["_bucket", "_sum", "_count", "_created"],
        _ => &[],
    }
}

fn valid_type(ty: &str, format: Format) -> bool {
    match format {
        Format::Prometheus => matches!(
            ty,
            "counter" | "gauge" | "summary" | "histogram" | "untyped"
        ),
        Format::OpenMetrics => matches!(
            ty,
            "counter"
                | "gauge"
                | "summary"
                | "histogram"
                | "unknown"
                | "gaugehistogram"
                | "stateset"
                | "info"
        ),
    }
}

/// Parse and validate. Returns a description of the first violation.
pub fn validate(text: &str, format: Format) -> Result<Exposition, String> {
    let mut body = text;
    if format == Format::OpenMetrics {
        body = text
            .strip_suffix("# EOF\n")
            .ok_or("OpenMetrics exposition must end with `# EOF\\n`")?;
        if body.contains("# EOF") {
            return Err("`# EOF` before the end".into());
        }
    }
    if body.contains('\r') {
        return Err("carriage return in exposition".into());
    }
    let mut families: Vec<Family> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    let mut finished: HashSet<usize> = HashSet::new();
    let mut current: Option<usize> = None;
    let mut series_seen: HashSet<(String, Vec<(String, String)>)> = HashSet::new();

    let family_slot =
        |name: &str, families: &mut Vec<Family>, index: &mut HashMap<String, usize>| -> usize {
            if let Some(i) = index.get(name) {
                return *i;
            }
            families.push(Family {
                name: name.to_owned(),
                ..Family::default()
            });
            let _ = index.insert(name.to_owned(), families.len() - 1);
            families.len() - 1
        };

    for line in body.lines() {
        if line.is_empty() {
            if format == Format::OpenMetrics {
                return Err("empty line in OpenMetrics exposition".into());
            }
            continue;
        }
        if let Some(meta) = line.strip_prefix('#') {
            let mut parts = meta.splitn(4, ' ');
            let _ = parts.next(); // "" before the space after '#'
            let keyword = parts.next().unwrap_or_default();
            let name = parts.next().unwrap_or_default();
            let rest = parts.next();
            let is_meta = matches!(keyword, "TYPE" | "HELP")
                || (format == Format::OpenMetrics && keyword == "UNIT");
            if !is_meta {
                if format == Format::OpenMetrics {
                    return Err(format!("non-metadata comment line {line:?}"));
                }
                continue;
            }
            if !is_metric_name(name) {
                return Err(format!("invalid name in {line:?}"));
            }
            let i = family_slot(name, &mut families, &mut index);
            if finished.contains(&i) || (current != Some(i) && !families[i].samples.is_empty()) {
                return Err(format!("metadata for {name} after its samples"));
            }
            if current.is_some_and(|c| c != i) {
                let _ = finished.insert(current.unwrap_or(i));
            }
            current = Some(i);
            let fam = &mut families[i];
            match keyword {
                "TYPE" => {
                    if !fam.ty.is_empty() {
                        return Err(format!("second TYPE line for {name}"));
                    }
                    let ty = rest.unwrap_or_default();
                    if !valid_type(ty, format) {
                        return Err(format!("invalid type {ty:?} for {name}"));
                    }
                    fam.ty = ty.to_owned();
                }
                "HELP" => {
                    if fam.help.is_some() {
                        return Err(format!("second HELP line for {name}"));
                    }
                    let help = rest.unwrap_or_default();
                    let mut chars = help.chars();
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            let next = chars.next().ok_or("dangling escape in HELP")?;
                            let ok = match format {
                                Format::Prometheus => matches!(next, '\\' | 'n'),
                                Format::OpenMetrics => matches!(next, '\\' | 'n' | '"'),
                            };
                            if !ok {
                                return Err(format!("invalid escape in HELP for {name}"));
                            }
                        } else if c == '"' && format == Format::OpenMetrics {
                            return Err(format!("unescaped quote in HELP for {name}"));
                        }
                    }
                    fam.help = Some(help.to_owned());
                }
                _ => {
                    if fam.unit.is_some() {
                        return Err(format!("second UNIT line for {name}"));
                    }
                    let unit = rest.unwrap_or_default();
                    if !unit.is_empty() && !name.ends_with(&format!("_{unit}")) {
                        return Err(format!("UNIT {unit} is not a suffix of {name}"));
                    }
                    fam.unit = Some(unit.to_owned());
                }
            }
            continue;
        }

        let sample = parse_sample(line, format)?;
        // Resolve the sample to a family: the current family first, then by
        // stripping known suffixes.
        let mut resolved = None;
        if let Some(c) = current {
            let fam = &families[c];
            if allowed_suffixes(&fam.ty, format)
                .iter()
                .any(|sfx| sample.name == format!("{}{sfx}", fam.name))
            {
                resolved = Some(c);
            }
        }
        let i = if let Some(i) = resolved {
            i
        } else {
            if format == Format::OpenMetrics {
                // Untyped samples are not allowed to slip in after
                // metadata of another family with a clashing name.
                if index.contains_key(&sample.name) {
                    return Err(format!("sample {} does not fit its family", sample.name));
                }
            }
            let i = family_slot(&sample.name, &mut families, &mut index);
            if families[i].ty.is_empty() {
                families[i].ty = if format == Format::OpenMetrics {
                    "unknown"
                } else {
                    "untyped"
                }
                .to_owned();
            }
            i
        };
        if finished.contains(&i) {
            return Err(format!("family of {} is interleaved", sample.name));
        }
        if current.is_some_and(|c| c != i) {
            let _ = finished.insert(current.unwrap_or(i));
        }
        current = Some(i);
        let mut key_labels = sample.labels.clone();
        key_labels.sort();
        if !series_seen.insert((sample.name.clone(), key_labels)) {
            return Err(format!("duplicate series {line:?}"));
        }
        families[i].samples.push(sample);
    }

    // Name-clash rules across families: every sample name belongs to
    // exactly one family.
    let mut owners: HashMap<String, String> = HashMap::new();
    for fam in &families {
        let mut names: Vec<String> = allowed_suffixes(&fam.ty, format)
            .iter()
            .map(|sfx| format!("{}{sfx}", fam.name))
            .collect();
        names.push(fam.name.clone());
        for n in names {
            if let Some(other) = owners.insert(n.clone(), fam.name.clone()) {
                if other != fam.name {
                    return Err(format!("families {other} and {} clash on {n}", fam.name));
                }
            }
        }
    }

    for fam in &families {
        check_family(fam, format)?;
    }
    Ok(Exposition { families })
}

fn label_value<'a>(s: &'a Sample, key: &str) -> Option<&'a str> {
    s.labels
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}

fn without(s: &Sample, key: &str) -> Vec<(String, String)> {
    let mut l: Vec<(String, String)> = s.labels.iter().filter(|(k, _)| k != key).cloned().collect();
    l.sort();
    l
}

fn check_family(fam: &Family, format: Format) -> Result<(), String> {
    for s in &fam.samples {
        if s.exemplar.is_some() {
            let ok = match fam.ty.as_str() {
                "counter" => s.name == format!("{}_total", fam.name),
                "histogram" => s.name == format!("{}_bucket", fam.name),
                _ => false,
            };
            if !ok {
                return Err(format!("exemplar on {} not allowed", s.name));
            }
            check_exemplar(s.exemplar.as_deref().unwrap_or_default())?;
        }
        if s.labels.iter().any(|(k, _)| k.starts_with("__")) {
            return Err(format!("reserved label on {}", s.name));
        }
    }
    match fam.ty.as_str() {
        "counter" => {
            for s in &fam.samples {
                if s.value.is_nan() || s.value < 0.0 {
                    return Err(format!(
                        "counter {} has invalid value {}",
                        s.name, s.raw_value
                    ));
                }
            }
        }
        "summary" => {
            for s in &fam.samples {
                if s.name == fam.name {
                    let q = label_value(s, "quantile")
                        .ok_or_else(|| format!("summary sample {} without quantile", s.name))?;
                    let q = parse_number(q)?;
                    if !(0.0..=1.0).contains(&q) {
                        return Err(format!("quantile {q} out of range"));
                    }
                } else if label_value(s, "quantile").is_some() {
                    return Err(format!("quantile label on {}", s.name));
                }
                let is_count = s.name == format!("{}_count", fam.name);
                let is_sum = s.name == format!("{}_sum", fam.name);
                if is_count && (s.raw_value.contains('.') || s.raw_value.contains('e')) {
                    return Err(format!("non-integer count {}", s.raw_value));
                }
                if (is_sum || is_count) && (s.value.is_nan() || s.value < 0.0) {
                    return Err(format!("negative or NaN {} in summary", s.name));
                }
            }
        }
        "histogram" => check_histogram(fam, format)?,
        _ => {}
    }
    Ok(())
}

fn check_exemplar(text: &str) -> Result<(), String> {
    if !text.starts_with('{') {
        return Err(format!("exemplar without label set: {text:?}"));
    }
    let (labels, rest) = parse_labels(text)?;
    let chars: usize = labels
        .iter()
        .map(|(k, v)| k.chars().count() + v.chars().count())
        .sum();
    if chars > 128 {
        return Err("exemplar labels longer than 128 characters".into());
    }
    let rest = rest.strip_prefix(' ').ok_or("exemplar without value")?;
    let mut fields = rest.split(' ');
    let _ = parse_number(fields.next().unwrap_or_default())?;
    if let Some(ts) = fields.next() {
        let _ = parse_number(ts)?;
    }
    Ok(())
}

fn check_histogram(fam: &Family, format: Format) -> Result<(), String> {
    // Group buckets by series (labels without `le`).
    let mut series: BTreeMap<Labels, Vec<(f64, f64)>> = BTreeMap::new();
    let mut counts: BTreeMap<Vec<(String, String)>, f64> = BTreeMap::new();
    let mut sums: BTreeMap<Vec<(String, String)>, f64> = BTreeMap::new();
    let bucket_name = format!("{}_bucket", fam.name);
    let count_name = format!("{}_count", fam.name);
    let sum_name = format!("{}_sum", fam.name);
    for s in &fam.samples {
        if s.name == bucket_name {
            let le =
                label_value(s, "le").ok_or_else(|| format!("bucket without le in {}", fam.name))?;
            let le = parse_number(le)?;
            if le.is_nan() {
                return Err("NaN le".into());
            }
            series
                .entry(without(s, "le"))
                .or_default()
                .push((le, s.value));
        } else if label_value(s, "le").is_some() {
            return Err(format!("le label on {}", s.name));
        } else if s.name == count_name {
            let _ = counts.insert(without(s, "le"), s.value);
        } else if s.name == sum_name {
            let _ = sums.insert(without(s, "le"), s.value);
        }
    }
    for labels in counts.keys().chain(sums.keys()) {
        if !series.contains_key(labels) {
            return Err(format!("{}: _sum/_count without buckets", fam.name));
        }
    }
    for (labels, buckets) in &series {
        let mut prev_le = f64::NEG_INFINITY;
        let mut prev_count = 0.0;
        for (le, count) in buckets {
            if *le <= prev_le {
                return Err(format!("{}: buckets not sorted by le", fam.name));
            }
            if *count < prev_count {
                return Err(format!("{}: buckets not cumulative", fam.name));
            }
            prev_le = *le;
            prev_count = *count;
        }
        let Some((last_le, inf_count)) = buckets.last() else {
            return Err("histogram without buckets".into());
        };
        if !last_le.is_infinite() {
            return Err(format!("{}: no +Inf bucket", fam.name));
        }
        let negative = buckets.iter().any(|(le, _)| *le < 0.0);
        let has_sum = sums.contains_key(labels);
        let has_count = counts.contains_key(labels);
        if format == Format::OpenMetrics {
            if negative && has_sum {
                return Err(format!("{}: _sum with negative buckets", fam.name));
            }
            if has_sum != has_count {
                return Err(format!("{}: _count present iff _sum present", fam.name));
            }
            if let Some(sum) = sums.get(labels) {
                if *sum < 0.0 || sum.is_nan() {
                    return Err(format!("{}: negative or NaN _sum", fam.name));
                }
            }
        }
        if let Some(count) = counts.get(labels) {
            if (count - inf_count).abs() > f64::EPSILON {
                return Err(format!(
                    "{}: _count {count} != +Inf bucket {inf_count}",
                    fam.name
                ));
            }
        }
    }
    Ok(())
}

#[test]
fn validator_rejects_known_bad_expositions() {
    let bad_prometheus = [
        // Two TYPE lines for one name (the 1.x cross-kind collision).
        "# TYPE dup counter\ndup 1\n# TYPE dup gauge\ndup 2\n",
        // A timer declared histogram without buckets (1.x).
        "# TYPE lat histogram\nlat_count 1\nlat_sum_seconds 0.1\n",
        // Injected line from an unescaped label key.
        "inj{bad\"key} 1\n",
        "a{k=\"v\n\"} 1\n",
        "x 1\ny 2\nx 3\n",
        "# TYPE h histogram\nh_bucket{le=\"1\"} 3\nh_bucket{le=\"+Inf\"} 2\nh_count 2\n",
        "# TYPE c counter\nc -1\n",
        "bad-name 1\n",
        "x 1\nx 1\n",
    ];
    for text in bad_prometheus {
        assert!(
            validate(text, Format::Prometheus).is_err(),
            "accepted: {text:?}"
        );
    }
    let bad_openmetrics = [
        "# TYPE c counter\nc_total 1\n",
        "# TYPE x gauge\n# UNIT x seconds\nx 1\n# EOF\n",
        "# TYPE c counter\nc 1\n# EOF\n",
        "# comment\n# EOF\n",
        "# TYPE g gauge\ng 1 # {a=\"b\"} 1\n# EOF\n",
        "# TYPE h histogram\nh_bucket{le=\"-1\"} 0\nh_bucket{le=\"+Inf\"} 1\nh_sum -2\nh_count 1\n# EOF\n",
        "# TYPE foo counter\nfoo_total 1\n# TYPE foo_created gauge\nfoo_created 1\n# EOF\n",
    ];
    for text in bad_openmetrics {
        assert!(
            validate(text, Format::OpenMetrics).is_err(),
            "accepted: {text:?}"
        );
    }
    let good = "# HELP c Things\\n done\n# TYPE c counter\nc{a=\"x\\\"y\"} 1\n# TYPE h histogram\nh_bucket{le=\"0.5\"} 1\nh_bucket{le=\"+Inf\"} 2\nh_sum 3.0\nh_count 2\n";
    assert!(validate(good, Format::Prometheus).is_ok());
}
