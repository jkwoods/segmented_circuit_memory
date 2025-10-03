# Memory in Arkworks
**Alert!! Please see [github.com/eniac/segmented-memory](github.com/eniac/segmented-memory) for an up to date, maintainted version of this Repo!**

This repo contains R1CS circuits, written in [arkworks](https://github.com/arkworks-rs) for several types of memory:
1. A version of [Nebula's memory construction](https://eprint.iacr.org/2024/1605.pdf) , that needs our [modified commitmit-carrying IVC version of Nova](https://github.com/jkwoods/Nova/tree/main)
2. A version of [Hekaton's memory construction](https://dl.acm.org/doi/pdf/10.1145/3658644.3690282?casa_token=PPpYVYkg0-wAAAAA:JOAcsNOzp071hfZpHEXRquOWhuMQq6NZMpBMKYcWGSO-7GF5S7s01zn_3XCaxYr5a90JY3v5KuAr), that needs our modified Nova.
3. Hash stack memory with both the [Poseidon hash](https://www.usenix.org/system/files/sec21-grassi.pdf) and a public coin hash.
4. A version of the [nlookup protocol](https://eprint.iacr.org/2023/573.pdf) (that is, the nlookup verifier is made noninteractive and expressed as R1CS) that incorporates hybrid tables and projection optimizations as described [here](https://eprint.iacr.org/2023/1886.pdf).

It also contains code for converting between arkworks and [bellperson](https://github.com/filecoin-project/bellperson) (the circuit library used in Nova).
