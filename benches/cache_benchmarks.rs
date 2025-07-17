// this_file: benches/cache_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fontgrepc::cache::FontCache;
use fontgrepc::font::FontInfo;
use fontgrepc::query::QueryCriteria;
use std::collections::HashMap;
use tempfile::NamedTempFile;

fn create_benchmark_font_info(name: &str, variable: bool) -> FontInfo {
    FontInfo {
        name_string: name.to_string(),
        is_variable: variable,
        axes: if variable { vec!["wght".to_string(), "wdth".to_string()] } else { vec![] },
        features: vec!["kern".to_string(), "liga".to_string(), "smcp".to_string()],
        scripts: vec!["latn".to_string(), "cyrl".to_string()],
        tables: vec!["GPOS".to_string(), "GSUB".to_string(), "GDEF".to_string()],
        charset_string: "abcdefghijklmnopqrstuvwxyz".to_string(),
    }
}

fn create_test_cache() -> (FontCache, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let cache_path = temp_file.path().to_str().unwrap();
    let cache = FontCache::new(Some(cache_path)).unwrap();
    (cache, temp_file)
}

fn benchmark_cache_creation(c: &mut Criterion) {
    c.bench_function("cache_creation", |b| {
        b.iter(|| {
            let temp_file = NamedTempFile::new().unwrap();
            let cache_path = temp_file.path().to_str().unwrap();
            let cache = FontCache::new(Some(cache_path)).unwrap();
            black_box(cache);
        });
    });
}

fn benchmark_single_font_insert(c: &mut Criterion) {
    c.bench_function("single_font_insert", |b| {
        b.iter_with_setup(
            || create_test_cache(),
            |(cache, _temp_file)| {
                let font_info = create_benchmark_font_info("Test Font", false);
                cache.batch_update_fonts(vec![
                    (format!("/path/to/font.ttf"), font_info, 12345, 1000)
                ]).unwrap();
            },
        );
    });
}

fn benchmark_batch_font_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_font_insert");
    
    for batch_size in [10, 100, 1000].iter() {
        group.bench_with_input(
            format!("batch_size_{}", batch_size),
            batch_size,
            |b, &batch_size| {
                b.iter_with_setup(
                    || {
                        let (cache, temp_file) = create_test_cache();
                        let mut fonts = Vec::new();
                        for i in 0..batch_size {
                            let font_info = create_benchmark_font_info(
                                &format!("Font {}", i),
                                i % 2 == 0,
                            );
                            fonts.push((
                                format!("/path/to/font{}.ttf", i),
                                font_info,
                                12345 + i as i64,
                                1000,
                            ));
                        }
                        (cache, temp_file, fonts)
                    },
                    |(cache, _temp_file, fonts)| {
                        cache.batch_update_fonts(fonts).unwrap();
                    },
                );
            },
        );
    }
    group.finish();
}

fn benchmark_font_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("font_search");
    
    // Setup cache with test data
    let (cache, _temp_file) = create_test_cache();
    let mut fonts = Vec::new();
    for i in 0..1000 {
        let font_info = create_benchmark_font_info(&format!("Font {}", i), i % 2 == 0);
        fonts.push((
            format!("/path/to/font{}.ttf", i),
            font_info,
            12345 + i as i64,
            1000,
        ));
    }
    cache.batch_update_fonts(fonts).unwrap();
    
    group.bench_function("search_all_fonts", |b| {
        b.iter(|| {
            let paths = cache.get_all_font_paths().unwrap();
            black_box(paths);
        });
    });
    
    group.bench_function("search_by_single_feature", |b| {
        b.iter(|| {
            let results = cache.query_by_features(&["kern".to_string()]).unwrap();
            black_box(results);
        });
    });
    
    group.bench_function("search_by_multiple_features", |b| {
        b.iter(|| {
            let results = cache.query_by_features(&[
                "kern".to_string(),
                "liga".to_string(),
            ]).unwrap();
            black_box(results);
        });
    });
    
    group.bench_function("search_variable_fonts", |b| {
        b.iter(|| {
            let mut criteria = QueryCriteria::default();
            criteria.variable = true;
            let results = cache.query(&criteria).unwrap();
            black_box(results);
        });
    });
    
    group.bench_function("complex_search", |b| {
        b.iter(|| {
            let mut criteria = QueryCriteria::default();
            criteria.variable = true;
            criteria.features = vec!["kern".to_string()];
            criteria.scripts = vec!["latn".to_string()];
            let results = cache.query(&criteria).unwrap();
            black_box(results);
        });
    });
    
    group.finish();
}

fn benchmark_cache_statistics(c: &mut Criterion) {
    let (cache, _temp_file) = create_test_cache();
    let mut fonts = Vec::new();
    for i in 0..1000 {
        let font_info = create_benchmark_font_info(&format!("Font {}", i), i % 2 == 0);
        fonts.push((
            format!("/path/to/font{}.ttf", i),
            font_info,
            12345 + i as i64,
            1000,
        ));
    }
    cache.batch_update_fonts(fonts).unwrap();
    
    c.bench_function("cache_statistics", |b| {
        b.iter(|| {
            let stats = cache.get_statistics().unwrap();
            black_box(stats);
        });
    });
}

fn benchmark_font_info_retrieval(c: &mut Criterion) {
    let (cache, _temp_file) = create_test_cache();
    let font_info = create_benchmark_font_info("Test Font", true);
    cache.batch_update_fonts(vec![
        ("/path/to/font.ttf".to_string(), font_info, 12345, 1000)
    ]).unwrap();
    
    c.bench_function("font_info_retrieval", |b| {
        b.iter(|| {
            let info = cache.get_font_info("/path/to/font.ttf").unwrap();
            black_box(info);
        });
    });
}

fn benchmark_cache_cleanup(c: &mut Criterion) {
    let (cache, _temp_file) = create_test_cache();
    let mut fonts = Vec::new();
    for i in 0..100 {
        let font_info = create_benchmark_font_info(&format!("Font {}", i), i % 2 == 0);
        fonts.push((
            format!("/path/to/font{}.ttf", i),
            font_info,
            12345 + i as i64,
            1000,
        ));
    }
    cache.batch_update_fonts(fonts).unwrap();
    
    c.bench_function("cache_cleanup", |b| {
        b.iter_with_setup(
            || {
                let mut existing_paths = std::collections::HashSet::new();
                // Keep half the fonts
                for i in 0..50 {
                    existing_paths.insert(format!("/path/to/font{}.ttf", i));
                }
                existing_paths
            },
            |existing_paths| {
                let removed = cache.clean_missing_fonts(&existing_paths).unwrap();
                black_box(removed);
            },
        );
    });
}

fn benchmark_concurrent_access(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;
    
    c.bench_function("concurrent_font_insert", |b| {
        b.iter_with_setup(
            || create_test_cache(),
            |(cache, _temp_file)| {
                let cache = Arc::new(cache);
                let mut handles = vec![];
                
                for i in 0..4 {
                    let cache_clone = Arc::clone(&cache);
                    let handle = thread::spawn(move || {
                        let font_info = create_benchmark_font_info(&format!("Font {}", i), false);
                        cache_clone.batch_update_fonts(vec![
                            (format!("/path/to/font{}.ttf", i), font_info, 12345 + i as i64, 1000)
                        ]).unwrap();
                    });
                    handles.push(handle);
                }
                
                for handle in handles {
                    handle.join().unwrap();
                }
            },
        );
    });
}

criterion_group!(
    benches,
    benchmark_cache_creation,
    benchmark_single_font_insert,
    benchmark_batch_font_insert,
    benchmark_font_search,
    benchmark_cache_statistics,
    benchmark_font_info_retrieval,
    benchmark_cache_cleanup,
    benchmark_concurrent_access
);

criterion_main!(benches);