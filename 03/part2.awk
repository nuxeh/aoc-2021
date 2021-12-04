BEGIN {
	FS=""
	pass=0
	regexp="1\\.\\*"
}

$0 ~ regexp {
	print $0
	for (i=1; i<=NF; i++) {
		b[i] += $i
	}
}

END {
	for (i=1; i<=NF; i++) {
		if (b[i]<NR/2)
			epsilon += lshift(1, NF-i)
		else
			gamma += lshift(1, NF-i)
	}

	print "gamma=" gamma
	print "epsilon=" epsilon
	print "power=" epsilon * gamma
}
