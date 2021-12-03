BEGIN {
	FS=""	
}

{
	for (i=1; i<=NF; i++) {
		print i
		b[i] += $i
	}
}

END {
	print b[1]
	print b[2]
	print b[3]
	print b[4]
	print b[5]
}
