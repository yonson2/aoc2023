use aoc2023::{
    day01, day01_revised, day02, day03, day04, day05, day05_revised, day06, timer::Timer, tools,
};

fn main() {
    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day01");
        day01::solve(data);
    }
    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day01");
        day01_revised::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day02");
        day02::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day03");
        day03::solve(data)
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day04");
        day04::solve(data)
    }

    // {
    //     let _timer = Timer::new();
    //     let data = tools::read_string("data/day05");
    //     day05::solve(data)
    // }
    // {
    //     let _timer = Timer::new();
    //     let data = tools::read_string("data/day05");
    //     day05_revised::solve(data)
    // }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day06");
        day06::solve(data)
    }
}
