## balanced-edges
The family of [ReCombination Markov Chains](https://hdsr.mitpress.mit.edu/pub/1ds8ptxu) (ReCom) have been used in redistricting to perform outlier analysis on gerrymandered maps.
At a high level, the precincts of a state can be represented as a dual graph. 
Adjacent districts get randomly merged, then split by drawing a spanning tree and finding a balanced edge to cut such that the population constraints of the redistricting problem are respected.
This method allows one to sample the space of plausible districting plans and make certain statistical claims about gerrymandered maps.

The original ReCom MCMC algo is not reversible. 
However, with a slight modification (by keeping track of certain probabilities), one can produce a reversible ReCom MCMC sampling method. [^1]
In order to sample properly, the reversible ReCom variant has to know the maximum length of a balanced edge seam, *M*, that can possibly occur while sampling.

Previously this was done through trial and error, however, with a recursive DFS search, we can establish firm upper bounds on *M* by reducing this problem down to a variant of the longest path problem.
The runtime for this implementation is somewhere around `O(v*2^d)` (it's DFS search for the longest path in the graph, not DFS for a particular node).

Note: Since redistricting occurs on planar graphs, this is an [NP-hard problem](https://en.wikipedia.org/wiki/Longest_path_problem).

## Usage

Build for release:
```
bash release.sh
```

Example usage (for MA senate, 40 districts, at 0.05 epsilon):
```
./target/release/balanced-edges -f ma-vtd-connected.json -d 40 -t 0.05
```
This produces an upper bound of `9` in 23 seconds with 10 cores on my laptop.

## Dual graphs
Download them from here: http://data.mggg.org.s3-website.us-east-2.amazonaws.com/dual-graphs/
or generate them yourself.

[^1]: This result is by Sarah Cannon, Moon Duchin, Dana Randall, and Parker Rule. (Preprint will be coming soon)
