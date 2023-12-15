use aoc2023::{
    day01, day01_revised, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
    day12, day13, day14, day15, timer::Timer, tools,
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
    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day05");
        day05::solve(data)
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day06");
        day06::solve(data)
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day07");
        day07::solve(data)
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day08");
        day08::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day09");
        day09::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day10");
        day10::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day11");
        day11::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day12");
        day12::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day13");
        day13::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_lines("data/day14");
        day14::solve(data);
    }

    {
        let _timer = Timer::new();
        let data = tools::read_string("data/day15");
        day15::solve(data);
    }
}
