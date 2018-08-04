#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate pg_query_parser;

fn benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| {
            pg_query_parser::pg_query_parse(
                "
                SELECT DISTINCT foo, bar
                FROM sys.baz INNER JOIN (VALUES ('a', $1)) x ON TRUE
                WHERE col_a = '123'
                ORDER BY col_b;
                ",
            )
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
