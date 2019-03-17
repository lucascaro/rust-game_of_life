use clap::*;
pub fn get_matches<'a>() -> ArgMatches<'a> {
  App::new("Life")
    .version("1.0.0")
    .author("Lucas Caro")
    .about("The game is life")
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .multiple(true)
        .help("Sets the level of verbosity"),
    )
    .subcommand(
      SubCommand::with_name("edit")
        .about("starts in edit mode (interactive)")
        .arg(Arg::with_name("INFILE").required(true).help("Input file"))
        .arg(Arg::with_name("OUTFILE").required(true).help("Output file")),
    )
    .get_matches()
}
