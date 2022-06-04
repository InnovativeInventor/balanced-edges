# fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 100 ./target/release/balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.03 > logs/{/}.senate-3.log'
# fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 100 ./target/release/balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber senate) -t 0.01 > logs/{/}.senate-1.log'
fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 100 ./target/release/balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber house) -t 0.03 > logs/{/}.house-3.log'
fd vtd-connected.json dual-graphs -j 1 -x bash -c 'timeout 100 ./target/release/balanced-edges -f {} -d $(python python/seats_lookup.py {} --chamber house) -t 0.01 > logs/{/}.house-1.log'
