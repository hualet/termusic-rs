extern crate ncurses;

mod utils;

use std::path::Path;

use ncurses::*;

use utils::fs_utils::list_files;

fn main() {
    initscr();

    printw("Hello, world!");

    refresh();

    println!("{:?}", list_files(&Path::new("/Users/hualet/")));

    getch();

    endwin();
}
