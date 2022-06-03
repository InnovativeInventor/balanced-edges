fd connected.json -j 1 -x bash -c 'balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.05 > logs/{/}.senate-5.log'
