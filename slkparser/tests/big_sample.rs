use std::time::Instant;

#[cfg(test)]
mod big_sample {
    use std::time::Instant;
    use slkparser::SLKScanner;
    use slkparser::document::Document;
    use crate::elapsed_time;

    #[test]
    fn test_ability_data() {
        let now = Instant::now();
        let slk_reader = SLKScanner::open("../resources/slk/AbilityData.slk");
        let mut document = Document::default();
        document.load(slk_reader);
        elapsed_time(&now);
//        for _ in document.get_contents(){}
        elapsed_time(&now);

    }
}

fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {}:{}:{}::{}", hours, mins, seconds, millis);
}