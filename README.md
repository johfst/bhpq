# bhpq
Rust bounded-height priority queue. Ideally meant to be *mostly* a drop-in replacement for binaryheaps.

In-progress, but basic functionality works. need to write documentation, improve usability, ~~write unit tests~~.

- probably switch from linkedlists to vecs, which i'm guessing will improve performance b/c caching.
- min/max priority support
- resize
- treating the datastructure itself as an iter vs. doing into_iter? maybe just both
- uhh other useful functions from binaryheap
- docs
