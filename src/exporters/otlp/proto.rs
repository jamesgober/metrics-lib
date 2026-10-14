//! Hand-written protobuf encoder for `ExportMetricsServiceRequest`.
//!
//! Field numbers and wire types follow `opentelemetry/proto/metrics/v1`,
//! `common/v1`, `resource/v1` and `collector/metrics/v1` of
//! opentelemetry-proto. Only the messages this exporter emits are encoded.
//! Nested messages are written into scratch buffers and prefixed with their
//! length.

use super::{DataModel, HistogramPoint, MetricModel, Number, NumberPoint, SummaryPoint};
use crate::LabelSet;

const VARINT: u8 = 0;
const FIXED64: u8 = 1;
const LEN: u8 = 2;

#[derive(Default)]
struct Buf(Vec<u8>);

impl Buf {
    fn varint(&mut self, mut v: u64) {
        loop {
            #[allow(clippy::cast_possible_truncation)] // masked to 7 bits
            let byte = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.0.push(byte);
                return;
            }
            self.0.push(byte | 0x80);
        }
    }

    fn tag(&mut self, field: u32, wire: u8) {
        self.varint((u64::from(field) << 3) | u64::from(wire));
    }

    fn uint(&mut self, field: u32, v: u64) {
        self.tag(field, VARINT);
        self.varint(v);
    }

    fn boolean(&mut self, field: u32, v: bool) {
        self.uint(field, u64::from(v));
    }

    fn fixed64(&mut self, field: u32, v: u64) {
        self.tag(field, FIXED64);
        self.0.extend_from_slice(&v.to_le_bytes());
    }

    fn sfixed64(&mut self, field: u32, v: i64) {
        self.tag(field, FIXED64);
        self.0.extend_from_slice(&v.to_le_bytes());
    }

    fn double(&mut self, field: u32, v: f64) {
        self.tag(field, FIXED64);
        self.0.extend_from_slice(&v.to_le_bytes());
    }

    fn bytes(&mut self, field: u32, data: &[u8]) {
        self.tag(field, LEN);
        self.varint(data.len() as u64);
        self.0.extend_from_slice(data);
    }

    fn string(&mut self, field: u32, s: &str) {
        if !s.is_empty() {
            self.bytes(field, s.as_bytes());
        }
    }

    fn message(&mut self, field: u32, build: impl FnOnce(&mut Buf)) {
        let mut inner = Buf::default();
        build(&mut inner);
        self.bytes(field, &inner.0);
    }

    fn packed_fixed64(&mut self, field: u32, values: &[u64]) {
        if values.is_empty() {
            return;
        }
        let mut inner = Vec::with_capacity(values.len() * 8);
        for v in values {
            inner.extend_from_slice(&v.to_le_bytes());
        }
        self.bytes(field, &inner);
    }

    fn packed_double(&mut self, field: u32, values: &[f64]) {
        if values.is_empty() {
            return;
        }
        let mut inner = Vec::with_capacity(values.len() * 8);
        for v in values {
            inner.extend_from_slice(&v.to_le_bytes());
        }
        self.bytes(field, &inner);
    }

    fn key_value(&mut self, field: u32, key: &str, value: &str) {
        self.message(field, |kv| {
            kv.string(1, key);
            // AnyValue.string_value = 1; an empty string must still be set
            // so the value is not "unset".
            kv.message(2, |any| any.bytes(1, value.as_bytes()));
        });
    }

    fn attributes(&mut self, field: u32, labels: &LabelSet) {
        for (k, v) in labels.iter() {
            self.key_value(field, k, v);
        }
    }
}

/// Encode a full `ExportMetricsServiceRequest`.
pub(crate) fn encode_request(
    model: &[MetricModel],
    service_name: &str,
    resource: &LabelSet,
) -> Vec<u8> {
    let mut out = Buf::default();
    // ExportMetricsServiceRequest.resource_metrics = 1
    out.message(1, |rm| {
        // ResourceMetrics.resource = 1
        rm.message(1, |res| {
            // Resource.attributes = 1
            res.key_value(1, "service.name", service_name);
            for (k, v) in resource.iter().filter(|(k, _)| *k != "service.name") {
                res.key_value(1, k, v);
            }
        });
        // ResourceMetrics.scope_metrics = 2
        rm.message(2, |sm| {
            // ScopeMetrics.scope = 1
            sm.message(1, |scope| {
                scope.string(1, "metrics-lib");
                scope.string(2, env!("CARGO_PKG_VERSION"));
            });
            // ScopeMetrics.metrics = 2
            for metric in model {
                sm.message(2, |m| encode_metric(m, metric));
            }
        });
    });
    out.0
}

fn encode_metric(m: &mut Buf, metric: &MetricModel) {
    m.string(1, &metric.name);
    m.string(2, &metric.description);
    m.string(3, &metric.unit);
    match &metric.data {
        // Metric.gauge = 5; Gauge.data_points = 1
        DataModel::Gauge(points) => m.message(5, |g| {
            for p in points {
                g.message(1, |dp| encode_number_point(dp, p));
            }
        }),
        // Metric.sum = 7
        DataModel::Sum {
            points,
            monotonic,
            temporality,
        } => m.message(7, |s| {
            for p in points {
                s.message(1, |dp| encode_number_point(dp, p));
            }
            s.uint(2, u64::from(temporality.wire()));
            s.boolean(3, *monotonic);
        }),
        // Metric.histogram = 9
        DataModel::Histogram {
            points,
            temporality,
        } => m.message(9, |h| {
            for p in points {
                h.message(1, |dp| encode_histogram_point(dp, p));
            }
            h.uint(2, u64::from(temporality.wire()));
        }),
        // Metric.summary = 11
        DataModel::Summary(points) => m.message(11, |s| {
            for p in points {
                s.message(1, |dp| encode_summary_point(dp, p));
            }
        }),
    }
}

fn encode_number_point(dp: &mut Buf, p: &NumberPoint) {
    dp.fixed64(2, p.start);
    dp.fixed64(3, p.time);
    match p.value {
        Number::Double(v) => dp.double(4, v),
        Number::Int(v) => dp.sfixed64(6, v),
    }
    dp.attributes(7, &p.attributes);
}

fn encode_histogram_point(dp: &mut Buf, p: &HistogramPoint) {
    dp.fixed64(2, p.start);
    dp.fixed64(3, p.time);
    dp.fixed64(4, p.count);
    if let Some(sum) = p.sum {
        dp.double(5, sum);
    }
    dp.packed_fixed64(6, &p.bucket_counts);
    dp.packed_double(7, &p.explicit_bounds);
    dp.attributes(9, &p.attributes);
}

fn encode_summary_point(dp: &mut Buf, p: &SummaryPoint) {
    dp.fixed64(2, p.start);
    dp.fixed64(3, p.time);
    dp.fixed64(4, p.count);
    dp.double(5, p.sum);
    for (q, v) in &p.quantiles {
        dp.message(6, |qv| {
            qv.double(1, *q);
            qv.double(2, *v);
        });
    }
    dp.attributes(7, &p.attributes);
}

#[cfg(test)]
pub(crate) mod decode {
    //! Minimal protobuf reader used to check the encoder's output.

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum Value {
        Varint(u64),
        Fixed64(u64),
        Bytes(Vec<u8>),
    }

    pub(crate) fn fields(mut data: &[u8]) -> Vec<(u32, Value)> {
        let mut out = Vec::new();
        while !data.is_empty() {
            let key = varint(&mut data);
            let field = u32::try_from(key >> 3).unwrap();
            let value = match key & 7 {
                0 => Value::Varint(varint(&mut data)),
                1 => {
                    let (head, rest) = data.split_at(8);
                    data = rest;
                    Value::Fixed64(u64::from_le_bytes(head.try_into().unwrap()))
                }
                2 => {
                    let len = usize::try_from(varint(&mut data)).unwrap();
                    let (head, rest) = data.split_at(len);
                    data = rest;
                    Value::Bytes(head.to_vec())
                }
                other => panic!("unexpected wire type {other}"),
            };
            out.push((field, value));
        }
        out
    }

    fn varint(data: &mut &[u8]) -> u64 {
        let mut v = 0u64;
        let mut shift = 0;
        loop {
            let byte = data[0];
            *data = &data[1..];
            v |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return v;
            }
            shift += 7;
        }
    }

    pub(crate) fn bytes(fields: &[(u32, Value)], field: u32) -> Vec<Vec<u8>> {
        fields
            .iter()
            .filter(|(f, _)| *f == field)
            .filter_map(|(_, v)| match v {
                Value::Bytes(b) => Some(b.clone()),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn one(fields: &[(u32, Value)], field: u32) -> Value {
        fields.iter().find(|(f, _)| *f == field).map_or_else(
            || panic!("field {field} missing in {fields:?}"),
            |(_, v)| v.clone(),
        )
    }

    pub(crate) fn string(fields: &[(u32, Value)], field: u32) -> String {
        match one(fields, field) {
            Value::Bytes(b) => String::from_utf8(b).unwrap(),
            other => panic!("field {field} is {other:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decode::{Value, bytes, fields, one, string};
    use super::*;

    #[test]
    fn test_varint_encoding() {
        let mut b = Buf::default();
        b.varint(0);
        b.varint(1);
        b.varint(300);
        b.varint(u64::MAX);
        assert_eq!(&b.0[..4], &[0, 1, 0xac, 0x02]);
        assert_eq!(b.0.len(), 4 + 10);
    }

    #[test]
    fn test_number_point_round_trip() {
        let mut labels = LabelSet::new();
        let _ = labels.add("k", "v");
        let point = NumberPoint {
            attributes: labels,
            start: 11,
            time: 22,
            value: Number::Int(-5),
        };
        let mut b = Buf::default();
        encode_number_point(&mut b, &point);
        let f = fields(&b.0);
        assert_eq!(one(&f, 2), Value::Fixed64(11));
        assert_eq!(one(&f, 3), Value::Fixed64(22));
        assert_eq!(
            one(&f, 6),
            Value::Fixed64(u64::from_le_bytes((-5i64).to_le_bytes()))
        );
        let kv = fields(&bytes(&f, 7)[0]);
        assert_eq!(string(&kv, 1), "k");
        let any = fields(&bytes(&kv, 2)[0]);
        assert_eq!(string(&any, 1), "v");
    }

    #[test]
    fn test_empty_attribute_value_is_still_set() {
        let mut b = Buf::default();
        b.key_value(1, "k", "");
        let kv = fields(&bytes(&fields(&b.0), 1)[0]);
        let any = fields(&bytes(&kv, 2)[0]);
        assert_eq!(one(&any, 1), Value::Bytes(Vec::new()));
    }
}
