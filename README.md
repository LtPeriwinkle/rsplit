# rsplit
## This is SUPER BAD! it's not worth your time to use, and i'm starting work on a much better timer.
(almost workable) TUI speedrun timer written in rust\
theoretically crossplatform since it uses crossterm, however i have not tested it outside of linux\
probably works best in something like [terminator](https://github.com/gnome-terminator/terminator) since it can show the colors correctly\
**this is such a mess that i've taken a break from developing it**

## a rough roadmap for this project:
1. figure out how to update terminal how i want (**done**)
2. parse json split files (**done**)
3. get timer working (**done enough**)
4. keyboard inputs (*in progress*)
5. dynamic colors for ahead/behind/gold
6. multithreading (i.e. a timer and a render thread)
7. tool for creating splits
