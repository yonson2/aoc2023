use aoc2023::{day01, day01_revised, day02, day03, day04, timer::Timer, tools};

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

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day02");
        day02::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day03");
        day03::solve(data)
    }

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day04");
        day04::solve(data)
    }
}
