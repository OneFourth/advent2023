use advent2023::day::get;
use color_eyre::Result;

fn print_section(label: &str, output: Option<&str>, time: &mut std::time::Instant) {
    use color_eyre::owo_colors::OwoColorize;

    match output {
        Some(output) => println!(
            "{}: {}\nTook {}{}\n",
            label.bright_yellow(),
            output.bright_green(),
            time.elapsed().as_millis().bright_blue(),
            "ms".bright_blue(),
        ),
        None => println!(
            "{} done!\nTook {}{}\n",
            label.bright_yellow(),
            time.elapsed().as_millis().bright_blue(),
            "ms".bright_blue(),
        ),
    }

    *time = std::time::Instant::now();
}

fn print_day(day: u8) {
    use color_eyre::owo_colors::OwoColorize;

    println!("Day {}", day.bright_purple());
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let day: u8 = std::env::args()
        .nth(1)
        .expect("Need a day input")
        .parse()
        .expect("Invalid input");

    print_day(day);

    let mut now = std::time::Instant::now();

    let mut day = get(day);

    day.setup()?;
    print_section("Setup", None, &mut now);

    let part1 = day.part1()?;
    print_section("Part 1", Some(&part1), &mut now);

    let part2 = day.part2()?;
    print_section("Part 2", Some(&part2), &mut now);

    Ok(())
}
