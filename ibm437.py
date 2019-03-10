#!/usr/bin/python3

# Author: Sebastien Bechet
# Licence: GPLv3

# \r = CR = 0x0d = ^M
# \n = LN = 0x0a

# sudo pacman -S python-pillow
from PIL import Image


def printAscii(i, image, width=9, height=14):
    if i < 0x80:
        print("\n# 0x{0:02X} ({1:c}):\n".format(i, i))
    else:
        print("\n# 0x{0:02X} (.):\n".format(i))
    for y in range(height):
        for x in range(width):
            print(image[y*width+x], end='')
        print()


def getImage(i, image, width=9, height=14, reverse=False):
    if reverse:
        foreground = b'\x00'
        background = b'\xff'
    else:
        foreground = b'\xff'
        background = b'\x00'

    im = Image.new('L', (width, height))
# not working?
#    im.putalpha(int.from_bytes(background, byteorder='little'))
    image2 = bytearray(image, 'utf-8')
    image2 = image2.replace(b'0', background).replace(b'1', foreground)
    im.putdata(image2)
    return im


def saveImage(i, image, width=9, height=14, end="normal", reverse=False):
    im = getImage(i, image, width, height, reverse)
    im.save("font_{}_{}_{}_{:02x}.png".format(width, height, end, i))
    return im


with open('IBM_5788005_AM9264_1981_CGA_MDA_CARD.BIN', mode='rb') as file:
    fileContent = file.read()

##############################################################################
# First font 8x14 (MDA)
for i in range(256):
    top = fileContent[8*i:8*(i+1)]
    bottom = fileContent[0x0800+8*i:0x0800+8*(i+1)]
    elem = top + bottom
    image = ''

    for j in range(0, 14):
        image = image + "{0:08b}".format(elem[j])
        # For characters C0h-DFh, the ninth pixel column
        # is a duplicate of the eight. For others, it's blank.
        if i & 0xE0 == 0xC0:
            image = image + "{0:08b}".format(elem[j])[7]
        else:
            image = image + '0'

    printAscii(i, image)
    saveImage(i, image, reverse=True)

##############################################################################
# Second font 8x8 (CGA)
for i in range(256):
    elem = fileContent[0x1000+8*i:0x1000+8*(i+1)]
    image = ''

    for j in range(0, 8):
        image = image + "{0:08b}".format(elem[j])

    printAscii(i, image, width=8, height=8)
    saveImage(i, image, width=8, height=8, reverse=True)

##############################################################################
# Third font 8x8 (CGA)
for i in range(256):
    elem = fileContent[0x1800+8*i:0x1800+8*(i+1)]
    image = ''

    for j in range(0, 8):
        image = image + "{0:08b}".format(elem[j])

    printAscii(i, image, width=8, height=8)
    saveImage(i, image, width=8, height=8, end="bold", reverse=True)
