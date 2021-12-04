#!/usr/bin/env fish

rm -f out out2

# create initial data from stdin
while read line
	echo $line >> out
	echo $line >> out2
end

# run first pass
set pass 1
while test (cat out | wc -l) -gt 1
	echo \n $pass \n

	cat out | awk -f part2.awk -v pass=$pass -v stage=0 > out_
	mv out_ out
	set pass (math "$pass"+1)

	cat out
end

set oxygen_generator_rating (cat out)

# run second pass
set pass 1
while test (cat out2 | wc -l) -gt 1
	echo \n $pass \n

	cat out2 | awk -f part2.awk -v pass=$pass -v stage=1 > out2_
	mv out2_ out2
	set pass (math "$pass"+1)

	cat out2
end

set co2_scrubber_rating (echo "obase=10; ibase=2; (cat out2)" | bc)

echo oxygen_generator_rating=$oxygen_generator_rating
echo co2_scrubber_rating=$co2_scrubber_rating
