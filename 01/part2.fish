#!/usr/bin/env fish

aocf checkout 1 2021

set buf 0 0 0
set idx 0
set sum 0
set last 0
set count 0

for num in (aocf input)
#for num in (cat test.txt)
    set buf[(math "$idx"+1)] $num
    set idx (math \("$idx"+1\)%3)

    set sum (math "$buf[1]"+"$buf[2]"+"$buf[3]")
    echo $sum

    if test $sum -gt $last
	if test $last -ne 0 -a $buf[1] -ne 0 -a $buf[2] -ne 0 -a $buf[3] -ne 0
            echo +
            set count (math "$count"+1)
	end
    end

    if test $buf[1] -ne 0 -a $buf[2] -ne 0 -a $buf[3] -ne 0
        set last $sum
    end
end
echo $count

