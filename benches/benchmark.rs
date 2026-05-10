use aoc2019::{day1, day2};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| { 
        let day01_str = std::fs::read_to_string("data/day1.txt").unwrap();

        b.iter(|| { 
            let day01_reader = std::io::BufReader::new(day01_str.as_bytes());
            let _ = day1::run(day01_reader);  
        });
        let day02_str = std::fs::read_to_string("data/day2.txt").unwrap();

        b.iter(|| { 
            let day02_reader = std::io::BufReader::new(day02_str.as_bytes());
            let _ = day2::run(day02_reader);  
        });
        
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
