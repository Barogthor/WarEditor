extern crate bencher;
extern crate slkparser;

use bencher::benchmark_group;
use bencher::benchmark_main;
use bencher::Bencher;
use slkparser::SLKScanner;

fn bench_scanning_ability_data(b: &mut Bencher) {
    let bytes = std::fs::read("../../resources/slk/AbilityData.slk").unwrap();
    b.iter(|| {
        let scanner = SLKScanner::from_bytes(bytes.clone());
        for record in scanner {
            record.unwrap();
        }
    });
}

benchmark_group!(benches, bench_scanning_ability_data);
benchmark_main!(benches);
