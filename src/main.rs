use std::fs::File;
extern crate clap;
#[macro_use]
extern crate log;
extern crate ncurses;
extern crate simplelog;

mod defer;
mod grid;
mod sample;

use defer::Defer;
use grid::Grid;

fn main() {
    use clap::*;
    let matches = App::new("Life")
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
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    simplelog::WriteLogger::init(
        log_level,
        simplelog::Config::default(),
        File::create("debug.log").unwrap(),
    ).unwrap();
    setup_screen();
    let _teardown = Defer { f: teardown_screen };
    debug!("Some code");

    if let Some(matches) = matches.subcommand_matches("edit") {
        let infile = matches.value_of("INFILE").unwrap();
        let outfile = matches.value_of("OUTFILE").unwrap();
        start_edit_mode(infile, outfile);
    } else {
        // let grid = Grid::new(ncurses::COLS() as usize, ncurses::LINES() as usize);
        let mut grid = Grid::from_vec(sample::seed2(
            ncurses::COLS() as usize,
            ncurses::LINES() as usize,
            // 10,
            // 10,
        ));
        // debug!("Grid: {}", grid);
        grid.update();
        let mut ch = ncurses::getch();
        while ch != ncurses::KEY_F(1) {
            grid.step();
            grid.update();
            ch = ncurses::getch();
        }
    }
}

fn start_edit_mode(infile: &str, outfile: &str) {
    use ncurses::*;
    // let grid = Grid::new(ncurses::COLS() as usize, ncurses::LINES() as usize);
    // let mut grid = Grid::new(ncurses::COLS() as usize, ncurses::LINES() as usize);
    debug!("edit mode: {} {}", infile, outfile);
    let mut grid = Grid::from_file(infile);
    debug!("Grid: {}", grid);
    grid.update();
    mvprintw(LINES() - 1, 0, "Press F1 to save and exit");
    mvprintw(LINES() - 1, 30, "Press F2 to exit without saving");
    mvprintw(0, 0, "Click to edit");
    refresh();
    mousemask(BUTTON1_CLICKED as mmask_t, None);

    let mut ch = wget_wch(stdscr());
    loop {
        match ch {
            Some(WchResult::KeyCode(KEY_MOUSE)) => {
                /* Enable attributes and output message. */

                let mut mevent = ncurses::MEVENT {
                    id: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    bstate: 0,
                };
                if getmouse(&mut mevent as *mut ncurses::MEVENT) == ncurses::OK {
                    debug!("EVENT {} {}", mevent.x, mevent.y);
                    grid.flip_cell(mevent.x as usize, mevent.y as usize);
                }
            }
            Some(WchResult::KeyCode(c)) => {
                if c == KEY_F(1) {
                    grid.save_to_file(outfile);
                }
                debug!("Key pressed: {:?}", c);
                break;
            }
            Some(WchResult::Char(_c)) => {
                /* Enable attributes and output message. */
                printw("\nKey pressed: ");
                grid.step();
            }
            None => {
                printw("\nYou didn't enter a character in time!");
            }
        }
        grid.update();
        ch = wget_wch(stdscr());
    }
    // while ch != ncurses::KEY_F(1) {
    //     if ch
    // }
}

fn setup_screen() -> (usize, usize) {
    use ncurses::*;
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    mvprintw(LINES() - 1, 0, "Press F1 to exit");

    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    return (max_x as usize, max_y as usize);
}

fn teardown_screen() {
    debug!("tearing down!");
    ncurses::endwin();
}
