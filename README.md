# BOCS
BOCS stands for Blant Orbit Counting System, and is a small utility to extend the functionality of Wayne Hayes tool [BLANT](https://github.com/waynebhayes/BLANT)

BOCS allows one to extend the output of BLANT to produce a estimated count of orbit pairs in a network graph using a 
[Count-min Sketch](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch) data structure to produce a probabilistic
guess for the lower bound of the count using far less memory that a strict count (on the order of single GB instead of tens).
