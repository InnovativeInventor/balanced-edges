## balanced-edges
Build for release:
```
bash release.sh
```

Example usage (for MA senate, 40 districts, at 0.05 epsilon):
```
./target/release/balanced-edges -f ma-vtd-connected.json -d 40 -t 0.05
```

Runtime is somewhere around `O(v*2^d)`, since this is on planar graphs and this is an [NP-hard problem](https://en.wikipedia.org/wiki/Longest_path_problem).

## Dual graphs
Download them from here: http://data.mggg.org.s3-website.us-east-2.amazonaws.com/dual-graphs/
