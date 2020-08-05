#!/bin/bash

cargo run --release

montage -geometry +0+0 `ls /tmp/font_9_14_normal_*.png | sort` font_9_14_normal.png
rm -f /tmp/font_9_14_normal_??.png

montage -geometry +0+0 `ls /tmp/font_8_8_normal_*.png | sort` font_8_8_normal.png
rm -f /tmp/font_8_8_normal_??.png

montage -geometry +0+0 `ls /tmp/font_8_8_bold_*.png | sort` font_8_8_bold.png
rm -f /tmp/font_8_8_bold_??.png

convert font_9_14_normal.png -depth 1 gray:font_9_14_normal.raw
convert font_8_8_normal.png -depth 1 gray:font_8_8_normal.raw
convert font_8_8_bold.png -depth 1 gray:font_8_8_bold.raw
