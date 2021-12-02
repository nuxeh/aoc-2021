#!/usr/bin/env fish

set last (aocf input | head -n 1) 
for num in (aocf input)
    echo $num
    if test $num -gt $last
        set count (math "$count"+1)
    end
    set last $num
end
echo $count

