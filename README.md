# rsplit
(soon to be) TUI speedrun timer written in rust\
will be crossplatform since it will use crossterm\
probably works best in something like [terminator](https://github.com/gnome-terminator/terminator) since it can show the colors correctly\
I'm fairly new to rust and doing this as basically a learning exercise so feel free to tell me all the things I'm doing wrong.

## a rough roadmap for this project:
1. figure out how to update terminal how i want (*in progress*)
2. parse json split files (**finished in old version**)
3. get timer working
4. keyboard inputs
5. dynamic colors for ahead/behind/gold
6. multithreading (i.e. a timer and a render thread)
7. tool for creating splits
