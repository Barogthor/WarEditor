extern crate bencher;
extern crate slkparser;

use bencher::benchmark_group;
use bencher::benchmark_main;
use bencher::Bencher;
use slkparser::SLKScanner;

fn bench_scanning_ability_data(b: &mut Bencher) {
    b.iter(|| {
        let slk_reader = SLKScanner::open("../../resources/slk/AbilityData.slk").unwrap();
        for record in slk_reader {
            record.unwrap();
        }
    });
}

benchmark_group!(benches, bench_scanning_ability_data);
benchmark_main!(benches);
