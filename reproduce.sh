fd connected.json -j 1 -x bash -c 'balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.03 > logs/{/}.senate-3.log'
