# Open Questions

Open questions. Anyone should feel free to comment, answer, ask in this document.
This would also allow us to have a record for some concerns.

## Questions

- What if a user calls a public decrypt on a ciphertext not allowed on the L2?
    - Should we still charged him for it?
    - Could be because the co-processor didn’t have consensus on the L2
    - How do we make sure that L1 ACL and L2 ACL are synced?
        - There is inherent latency due to that
            - Let’s say you allow and ask for a decrypt in the same block, if the co-processors are slower then ZWS in reaching the L2 then we could have ZWS transaction reverted because the ACL wasn’t synced yet on the L2
- How are coprocessors parties incentivized to participate in the protocol and how do we prevent cheating? (FHE computations reward + stake and slash?)
