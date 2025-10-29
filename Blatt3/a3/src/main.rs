use clap::{App, Arg};
use std::{process, thread, time::Duration};

fn main() {
    let matches = App::new("exitcode")
        .version("1.0")
        .about("Beendet sich nach einer bestimmten Zeit mit einem angegebenen Exit-Code.")
        .arg(
            Arg::new("exit")
                .short('e')
                .long("exit")
                .value_name("CODE")
                .help("Exit-Code, mit dem das Programm beendet wird")
                .takes_value(true),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("SEKUNDEN")
                .help("Verzögerung in Sekunden vor dem Beenden (Standard: 0)")
                .takes_value(true),
        )
        .get_matches();

    if std::env::args().len() == 1 {
        App::new("exitcode")
            .version("1.0")
            .about("Beendet sich nach einer bestimmten Zeit mit einem angegebenen Exit-Code.")
            .arg(Arg::new("exit").short('e').long("exit").value_name("CODE"))
            .arg(
                Arg::new("delay")
                    .short('d')
                    .long("delay")
                    .value_name("SEKUNDEN"),
            )
            .print_help()
            .unwrap();
        println!();
        process::exit(0);
    }

    let exit_code: i32 = matches
        .value_of("exit")
        .unwrap_or("0")
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Fehler: Exit-Code muss eine ganze Zahl sein!");
            process::exit(1);
        });

    let delay_secs: u64 = matches
        .value_of("delay")
        .unwrap_or("0")
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Fehler: Delay muss eine ganze Zahl sein!");
            process::exit(1);
        });

    if delay_secs > 0 {
        thread::sleep(Duration::from_secs(delay_secs));
    }

    process::exit(exit_code);
}
