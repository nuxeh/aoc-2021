#!/usr/bin/env fish

rm -f out

# create initial data from stdin
while read line
	echo $line >> out
end

# run passes
set pass 1

while test (cat out | wc -l) -gt 1
	echo \n $pass \n
	cat out | awk -f part2.awk -v pass=$pass > out2
	mv out2 out
	cat out
	set pass (math "$pass"+1)
end
