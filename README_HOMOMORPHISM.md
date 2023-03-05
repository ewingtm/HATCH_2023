# README_HOMOMORPISM.

DISCLAIMER: This code is a toy: it's for fun, learning.

---

This code makes use of the 'Zama.ai' concrete library, which is a developer focused library
written in Rust that allows for homomorphic encryption by using TFHE. For the layperson, the
idea is that you can encrypt your data (which in this case is identifying genetic data) and then
do operations on the ciphertext like E[x] + E[y] or E[x] + 3, etc. After working on the ciphertext,
the keyholder can decrypt the ciphertext, with the correct operations being the result.

We've tried to implement something dead simple that a researcher (or more likely, a customer to a diagnostician)
might use: say, computing the hamming distance between two sequences. An example can be seen by running 
`cargo run --release --bin hamming`. In this example, you can compute the hamming distance between 
two sequences of nucleotides without revealing any specific information[^1] about the configuration of 
the nucleotides.

This can be expanded: we can build an adjacency matrix of all hamming distances of a set of
sequences. An example* (code is included for enc, but you'd truly need a proper parallel model)
is shown with some toy data. This distance matrix could be utilized for clustering, the building
of graphs, forming of a topological space, etc. See `src/bin/adj.rs`

Beyond what we've tried to do, there's potential for other avenues: dotproducts, convolution,
neural networks that work on homomorphic data (it's easy to quantize down to fixed point or int4,
the only real operations are matirx muls, activation functions can be approximated or binarized.)
Us mere mortals cannot come close to NNs, but there is *interesting* work done at quite a few places
on this problem.

Ultimately this solution is unusable due to performance. The performance is atrocious: it's not
even considerable. But neural networks were considered a joke before 2012's AlexNet which ran on GPUs.
On the other hand, people will pay $500 billion in computing power and specialized hardware for ChatGPT 
or Image Diffusion models: no one will pay near a sum of that for privacy. Until faster schemes are
developed, specialized hardware is developed, this is just something that's cool and will score you academic 
cred.

I'd send the interested layperson to the zama.ai whitepaper, the interested academic to CKKS, GSW, and
the broader lattice landscape that's received significant work the last few years due to NIST-PQC.

---

[^1]: You do learn some information however, the hamming distance. Repeated queries lead you to
a fun chosen plaintext attack. 1. The attacker controls one of the sequences. They query the oracle for the hamming distance
from the other sequence. 2. If the distance isn't zero, the attacker flips a bit at a starting index, then queries again.
If the hamming distance has dropped, the attacker knows the value at that position of the sequence. 3. This can be iteratively done, 
eventually arriving at the correct sequence. 
