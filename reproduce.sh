# fd vtd-connected.json dual-graphs -j 1 -x bash -c 'balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.03 > logs/{/}.senate-3.log'
fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 70 balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.03 > logs/{/}.senate-3.log'
fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 100 balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.01 > logs/{/}.senate-1.log'
