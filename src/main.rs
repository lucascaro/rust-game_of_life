extern crate clap;
#[macro_use]
extern crate log;
extern crate ncurses;
extern crate simplelog;

mod cli;
mod defer;
mod grid;
mod logger;
mod sample;

use defer::Defer;
use grid::Grid;

fn main() {
    let matches = cli::get_matches();
    logger::init(matches.occurrences_of("verbose"));

    setup_screen();
    let _teardown = Defer { f: teardown_screen };

    if let Some(matches) = matches.subcommand_matches("edit") {
        // Edit mode.
        let infile = matches.value_of("INFILE").unwrap();
        let outfile = matches.value_of("OUTFILE").unwrap();
        start_edit_mode(infile, outfile);
    } else {
        let mut grid = Grid::from_vec(sample::seed2(
            ncurses::COLS() as usize,
            ncurses::LINES() as usize,
        ));

        loop {
            // Core game loop.
            grid.update();
            ncurses::mvprintw(ncurses::LINES() - 1, 0, "Press q to save and exit");
            let ch = ncurses::getch();
            if ch == 'q' as i32 || ch == 'Q' as i32 {
                break;
            }
            grid.step();
        }
    }
}

fn start_edit_mode(infile: &str, outfile: &str) {
    use ncurses::*;

    debug!("edit mode: {} {}", infile, outfile);
    let mut grid = Grid::from_file(infile);
    debug!("Grid: {}", grid);
    mousemask(BUTTON1_CLICKED as mmask_t, None);

    loop {
        grid.update();
        mvprintw(LINES() - 1, 0, "Press F1 to save and exit");
        mvprintw(LINES() - 1, 30, "Press F2 to exit without saving");
        mvprintw(0, 0, "Click to edit");
        refresh();
        let ch = wget_wch(stdscr());
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
    }
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

    (max_x as usize, max_y as usize)
}

fn teardown_screen() {
    debug!("tearing down!");
    ncurses::endwin();
}
