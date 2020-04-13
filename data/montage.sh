#!/bin/bash

./ibm437.py

magick montage -geometry +0+0 `ls font_9_14_normal_*.png | sort` font_9_14_normal.png
rm -f font_9_14_normal_??.png

magick montage -geometry +0+0 `ls font_8_8_normal_*.png | sort` font_8_8_normal.png
rm -f font_8_8_normal_??.png

magick montage -geometry +0+0 `ls font_8_8_bold_*.png | sort` font_8_8_bold.png
rm -f font_8_8_bold_??.png

convert font_9_14_normal.png -depth 1 gray:font_9_14_normal.raw
convert font_8_8_normal.png -depth 1 gray:font_8_8_normal.raw
convert font_8_8_bold.png -depth 1 gray:font_8_8_bold.raw
