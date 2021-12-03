{
sum_0+=$0
sum_1+=$1
sum_2+=$2
sum_3+=$3
sum_4+=$4
} END { 
 print "raradasd"
print sum_0
print sum_1
print sum_2
print sum_3
print sum_4
if (sum_0 > 6) { print 1 } else { print 0 }
if (sum_1 > 6) { print 1 } else { print 0 }
if (sum_2 > 6) { print 1 } else { print 0 }
if (sum_3 > 6) { print 1 } else { print 0 }
if (sum_4 > 6) { print 1 } else { print 0 }
}
