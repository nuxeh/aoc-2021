BEGIN {
	FS=""
	b=0
}

{
	b += $pass

	if ($pass == 0)
		zeroes[z++] = $0
	else
		ones[o++] = $0
}

END {
	if (b > NF/2)
		for (i in ones) print ones[i]
	else
		for (i in zeroes) print zeroes[i]
}
