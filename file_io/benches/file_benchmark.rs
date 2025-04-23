use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufReader, Read, Write, BufWriter};
use std::path::Path;

fn file_benchmark(c: &mut Criterion) {
    // Create a temporary file for benchmarking
    let temp_file_path = "../test/benchmark.protected";
    let test_content = "This is some test content to benchmark file writing performance.\n".repeat(1000);

    // Create a test file with some content if it doesn't exist
    if !Path::new(temp_file_path).exists() {
        std::fs::write(temp_file_path, test_content).expect("Failed to create benchmark file");
    }

    // Benchmark file reading with File object and BufReader
    c.bench_function("read_file_bufreader", |b| {
        b.iter(|| {
            let file = File::open(temp_file_path).expect("Failed to open file");
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content).expect("Failed to read file");
            black_box(content);
        })
    });
    
    // Benchmark file writing with File object and BufWriter
    c.bench_function("write_file_bufwriter", |b| {
        b.iter(|| {
            let file = File::create(temp_file_path).expect("Failed to create file");
            let mut writer = BufWriter::new(file);
            writer.write_all(test_content.as_bytes()).expect("Failed to write file");
            writer.flush().expect("Failed to flush writer");
            black_box(());
        })
    });
        
    // Clean up the temporary file
    // std::fs::remove_file(temp_file_path).expect("Failed to clean up benchmark file");
    // Note: We're leaving the file for now in case you want to run benchmarks multiple times
}

criterion_group!(benches, file_benchmark);
criterion_main!(benches); 