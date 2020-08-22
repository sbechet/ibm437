# Original MDA Character ROM extractor

[source](http://www.minuszerodegrees.net/rom/rom.htm)

-   Make: IBM
-   Model: MDA/CGA
-   Function: Display adapters
-   Year: ?
-   Data Type: Font
-   Sum: 06DDC0
-   Chip Type: MK36000 (filename:AM9264)
-   Size: 8 KB
-   Copy: C
-   Source: modem7
-   Description/Comment: IBM part number on chip is 5788005

There are TWO CGA fonts in the ROM: a thin 5x7 "single dot" font located at 
offset 0x1000, and a thick 7x7 "double dot" font at offset 0x1800.

The latter is the default font, unless overridden by a jumper setting on the 
CGA card.

Each character is rendered in a box of 9×14 pixels, of which 7×11 depicts the 
character itself (the other pixels being used for space between character 
columns and lines). Some characters, such as the lowercase "m", are rendered 
eight pixels across.

An MDA produces an 80x25 text screen. Each character is 9 pixels wide and 
14 high, giving a 720x350 pixel resolution.

Although the characters are 9 pixels wide, the bitmaps in the ROM are only 8 
pixels. For characters C0h-DFh, the ninth pixel column is a duplicate of the 
eight; for others, it's blank.

