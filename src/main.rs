use aoc2023::{day01, timer::Timer, tools};

fn main() {
    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day01");
        day01::solve(data);
    }
}
