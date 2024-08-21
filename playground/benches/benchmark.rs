use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

// 使用reqwest库以阻塞方式请求http数据
fn request_http_data_blocking() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://www.rust-lang.org")?;
    let body = response.text()?;
    Ok(body)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("request_http_data_blocking", |b| b.iter(|| request_http_data_blocking()));
}

criterion_group!(benches1, criterion_benchmark);
criterion_main!(benches1);