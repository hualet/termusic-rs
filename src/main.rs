extern crate ncurses;

mod utils;

use std::path::Path;

use ncurses::*;

use utils::fs_utils::list_files;

fn main() {
    initscr();

    let files = list_files(&Path::new("/Users/hualet/"));

    for file in files {
    	printw(&file[..]);
    	printw("\n");
    }

    refresh();

    getch();
    endwin();
}
