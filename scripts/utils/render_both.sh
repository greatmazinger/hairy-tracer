#!/bin/bash
python3 src/btrace.py --scene scenes/showcase_dice.json --size 800x800 --outfile dice_final.bmp
python3 src/btrace.py --scene scenes/showcase_dice_ivory.json --size 800x800 --outfile dice_ivory_final.bmp
cp dice_final.bmp /home/bing/.gemini/antigravity-cli/brain/3b8421c9-d6a3-45a3-9014-32700019943f/.tempmediaStorage/dice_final.bmp
cp dice_ivory_final.bmp /home/bing/.gemini/antigravity-cli/brain/3b8421c9-d6a3-45a3-9014-32700019943f/.tempmediaStorage/dice_ivory_final.bmp
