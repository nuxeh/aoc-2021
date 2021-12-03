set l (math (aocf input | head -n 1 | wc -c)-2)

echo \{

for i in (seq 0 $l)
    echo sum_$i+=\$$i
end

echo \} END \{

for i in (seq 0 $l)
    echo print sum_$i
end

echo \}
