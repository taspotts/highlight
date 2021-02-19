#[macro_use]
extern crate clap;

use clap::{App, Arg};

use regex::Regex;
use std::io::Write;
use std::io::{self, BufRead, BufReader, ErrorKind, Read};
use std::process;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        // .arg(Arg::with_name("INPUT")
        //     .help("Sets the input file to use")
        //     .index(1))
        .arg(Arg::with_name("FILE").required(false))
        .arg(
            Arg::with_name("red")
                .require_equals(true)
                .takes_value(true)
                .long("red")
                .multiple(true),
        )
        .arg(
            Arg::with_name("green")
                .require_equals(true)
                .multiple(true)
                .takes_value(true)
                .long("green"),
        )
        //     .arg(Arg::with_name("colors")
        //         .help("some help text goes here")
        //         .required(true)
        //         .multiple(true)
        //     )
        //     .usage("the usage test")
        .get_matches();

    println!("{:#?}", matches);
    // if matches.is_present("red") {
    // println!("red count={}", matches.occurrences_of("colors"));
    // println!("{:?}", matches.values_of("colors").unwrap_or_default().collect::<Vec<_>>());
    // }

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // match matches.occurrences_of("red") {
    //     0 => println!("No red"),
    //     _ => println!("Lots of red, {:?}", matches.value_of("red"))

    // let reds = matches.occurrences_of("colors");
    // if reds > 0 {
    //     println!("red count={}", reds);
    //     println!("{:?}", matches.values_of("red").unwrap_or_default().collect::<Vec<&str>>());
    // } else {
    //     println!("NO reds");
    // }
    //
    // let greens = matches.occurrences_of("green");
    // if greens > 0 {
    //     println!("greens count={}", greens);
    //     println!("{:?}", matches.values_of("greens").unwrap_or_default().collect::<Vec<&str>>());
    // } else {
    //     println!("NO greens");
    // }

    print_by_line(io::stdin()).expect("Could not read from stdin");
}
fn print_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut stdout = io::stdout();

    let re = Regex::new(r"\d+").unwrap();

    for line in buffer.lines() {
        // println!("{:>6}  {}", idx + 1, Red.paint(line?))
        // let l = format!("\x1b[0;32m{}\x1b[0m", line?);
        let l = re.replace_all(&line?, "\x1b[34m$0\x1b[0m").to_string();
        /*

        https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html

                            FG  BG
        Black	            30  40
        Red	                31  41
        Green	            32  42
        Yellow	            33  43
        Blue	            34  44
        Magenta	            35  45
        Cyan	            36  46
        Bright Black (Gray)	90	100
        Bright Red	        91	101
        Bright Green        92	102
        Bright Yellow   	93	103
        Bright Blue	        94	104
        Bright Magenta	    95	105
        Bright Cyan	        96	106
        Bright White	    97	107

         */

        if let Err(e) = writeln!(stdout, "{}", l) {
            if e.kind() != ErrorKind::BrokenPipe {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }

    // let stdout = std::io::stdout();
    // let mut handle = stdout.lock();
    // if let Err(_) = writeln!(handle, ...) { return Ok(()); }

    Ok(())
}
