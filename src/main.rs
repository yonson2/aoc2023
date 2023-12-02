use aoc2023::{day01, day01_revised, timer::Timer, tools};

fn main() {
    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day01");
        day01::solve(data);
    }
    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day01");
        day01_revised::solve(data);
    }
}
