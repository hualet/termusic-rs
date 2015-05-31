extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    printw("Hello, world!");

    refresh();

    getch();

    endwin();
}
