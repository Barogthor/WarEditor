
extern crate slkparser;
extern crate bencher;

use bencher::Bencher;
use bencher::benchmark_group;
use bencher::benchmark_main;
use slkparser::SLKScanner;


fn bench_scanning_ability_data(b: &mut Bencher) {
    b.iter(|| {
        let mut slk_reader = SLKScanner::open("../resources/slk/AbilityData.slk");
        for _ in slk_reader{}
    });
}

benchmark_group!(benches, bench_scanning_ability_data);
benchmark_main!(benches);
