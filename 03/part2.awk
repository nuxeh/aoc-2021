BEGIN {
	FS=""
	b=0
	print "pass=" pass > "/dev/stderr"
}

{
	b += $pass

	if ($pass == 0)
		zeroes[z++] = $0
	else
		ones[o++] = $0
}

END {
	print b " / " NR/2 > "/dev/stderr"
	if (b >= NR/2)
		for (i in ones) print ones[i]
	else
		for (i in zeroes) print zeroes[i]
}
