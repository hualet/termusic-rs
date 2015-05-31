extern crate ncurses;

mod utils;
mod player;

use std::cmp::{max, min};
use std::path::Path;

use ncurses::*;

use player::Player;
use utils::fs_utils::list_files;

static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_HIGHLIGHT: i16 = 2;

fn show_files(files: &Vec<String>, current_index: i16) {
	clear();
	mv(0, 0);

	for index in 0..files.len() {
		let mut color_pair = COLOR_PAIR_DEFAULT;
		if (index as i16 == current_index) {
			color_pair = COLOR_PAIR_HIGHLIGHT;
		}

		attron(COLOR_PAIR(color_pair));
		printw(&files[index][..]);
		printw("\n");
		attroff(COLOR_PAIR(color_pair));
	}
}

fn main() {
	let mut current_index = 0;

    initscr();

    start_color();
    init_pair(COLOR_PAIR_DEFAULT, constants::COLOR_GREEN, constants::COLOR_BLACK);
    init_pair(COLOR_PAIR_HIGHLIGHT, constants::COLOR_RED, constants::COLOR_BLACK);

    let files = list_files(&Path::new("/Users/hualet/"));
    show_files(&files, current_index);

    refresh();

    let mut player = Player::new();
    player.play("/Users/hualet/Desktop/test.mp3");

    let mut key = getch();
    while key != 'q' as i32 {
    	if key == 'j' as i32 {
    		current_index = min(current_index + 1, (files.len() - 1) as i16);
    	} else if (key == 'k' as i32) {
    		current_index = max(current_index - 1, 0);
    	}

    	show_files(&files, current_index);

    	key = getch();
    }

    endwin();
}
