# rsplit
(soon to be) TUI speedrun timer written in rust\
only for linux (and theoretically OS X), as it uses [termion](https://docs.rs/termion/1.5.5/termion/) and [tui](https://docs.rs/tui/0.9.5/tui/)\
Works best in something like [terminator](https://github.com/gnome-terminator/terminator) since it can show the colors correctly\
If someone out there finds real documentation on how to use tui with crossterm (other than the five lines of code on the tui link I linked) tell me where please\
I'm fairly new to rust and doing this as basically a learning exercise so feel free to tell me all the things I'm doing wrong.

## a rough roadmap for this project:
1. figure out how to update terminal how i want (**done**)
2. parse json split files (*in progress*)
3. get timer working
4. keyboard inputs
5. dynamic colors for ahead/behind/gold
6. multithreading (i.e. a timer and a render thread)
7. tool for creating splits
