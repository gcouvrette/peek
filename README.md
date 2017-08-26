# peek
    This tool is used to peek at the content of a file
    without reading all of it in memory. Very useful
    for big files.

    Possible options:
    -m [width, height] : Multiline - Reads the file as a multiline file.
                                     Peek will read until `height` lines are found
                                     and will display the first `width` characters
                                     of each line. In this mode, it is possible
                                     to move the reading area using the arrow keys.
                                     Width and height are optional. Defaults are
                                     `width`: 80, `height`: 40

    -l [length] : Linear (*default) - Reads the file in a linear pattern, reading only
                                     `length` chars at a time. Use arrow keys to
                                     move in the file. Left/Up goes back in the buffer,
                                     right/down moves forward. Default is
                                     `length`: 3200


This tool was created because I was trying to read a 64Gb XML file (Wikipedia data dump) and 
no existing text editor was able to process the file. Either they tried to load the whole file in memory (which was too big)
or was compiled in 32 bits which cannot read files of more than 2Gb.

