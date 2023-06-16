use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use rand::Rng;

mod questions;

fn main() {
    println!("Guess the correct pronunciation for each Japanese Hiragana sign to earn points.");
    println!("To close the game type: exit");
    println!("-------------------------------------------------------------------------------");

    let _stdout = StandardStream::stdout(ColorChoice::Always);

    let mut error_text = ColorSpec::new();
    error_text.set_fg(Some(Color::Red));
    error_text.set_bold(true);

    let mut success_text = ColorSpec::new();
    success_text.set_fg(Some(Color::Green));
    success_text.set_bold(true);

    let mut exit_text = ColorSpec::new();
    exit_text.set_fg(Some(Color::Yellow));
    exit_text.set_bold(true);

    let mut score: u8 = 0;
    let questions = &questions::QUESTIONS;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    loop {
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(0..questions.len());

        println!("How do you pronounce {}?", questions[n1].character);

        let mut answer = String::new();
        let _bytes = std::io::stdin().read_line(&mut answer);

        if answer.trim() == "exit" {
            print_colored_text(&mut stdout, &exit_text, format!("Bye bye! You've ended with {} points.", score));
            break;
        }

        if answer.trim().to_lowercase() == questions[n1].answer {
            score += 1;
            print_colored_text(&mut stdout, &success_text, format!("That is correct! Your score is now: {}", score));
        } else {
            print_colored_text(&mut stdout, &error_text, format!("That's not correct. The correct answer is: {}", questions[n1].answer));
        }

        println!("------------------------");
    }
}

fn print_colored_text<W: WriteColor>(
    stdout: &mut W,
    color_spec: &ColorSpec,
    text: String,
) {
    stdout.set_color(color_spec).unwrap();
    writeln!(stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}
