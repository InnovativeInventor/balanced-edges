import sys

"""
Example usage:
    cat summary.log | python summary_parse.py > summary.csv
or
    rg "Final" logs/ | sort -u | python summary_parse.py > summary.csv
"""

print("filename,districts,tolerance,upper bound")
for line in sys.stdin:
    print(",".join(filter(lambda x: any(char.isdigit() for char in x), map(lambda x: x.split(":")[0], line.rstrip().split()))))
