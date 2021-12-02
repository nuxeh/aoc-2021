#!/usr/bin/env fish

echo set x 0
echo set y 0
echo set aim 0

while read line
    echo $line | sed 's/forward \([0-9]*\)/set x (math "$x"+\1); set y (math "$y"+\\\(\1\\\*"$aim"\\\))/' \
    | sed 's/down \([0-9]*\)/set aim (math "$aim"+\1)/' \
    | sed 's/up \([0-9]*\)/set aim (math "$aim"-\1)/'
end

echo 'echo x=$x'
echo 'echo y=$y'
echo 'echo (math "$x"\*"$y")'
