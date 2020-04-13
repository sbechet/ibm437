#!/bin/bash

./ibm437.py

magick montage -geometry +0+0 `ls font_9_14_normal_*.png | sort` font_9_14_normal.png
rm -f font_9_14_normal_??.png

magick montage -geometry +0+0 `ls font_8_8_normal_*.png | sort` font_8_8_normal.png
rm -f font_8_8_normal_??.png

magick montage -geometry +0+0 `ls font_8_8_bold_*.png | sort` font_8_8_bold.png
rm -f font_8_8_bold_??.png

