set l (math (cat test.txt | head -n 1 | wc -c)-2)
set n (math (cat test.txt | wc -l)/2)

echo \{

for i in (seq 0 $l)
    echo sum_$i+=\$$i
end

echo \} END \{

for i in (seq 0 $l)
    echo print sum_$i
end

for i in (seq 0 $l)
    echo "if (sum_$i > $n) { print 1 } else { print 0 }"
end

echo \}
