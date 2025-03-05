// macro_rules! auto_call_collect {
//     () => {};
//     () => {};
// }

#[macro_export]
macro_rules! show_time_cost {
    ($func:expr $(, $args:expr)* ) => {{
        let start = std::time::Instant::now();

        let result = $func($($args),*);

        let cost = start.elapsed();
        println!(
            "###>>> Time cost: {}ns = {}us = {}ms",
            cost.as_nanos(),
            cost.as_micros(),
            cost.as_millis()
        );
        result
    }};
}