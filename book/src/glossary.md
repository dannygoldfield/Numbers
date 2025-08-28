# Glossary

## TXID

**Transaction ID.** Unique hash of a Bitcoin transaction. In SegWit, the
TXID excludes witness data.

## wtxid

**Witness Transaction ID.** Transaction ID including witness (SegWit)
data. Used in block validation.

## SegWit (Segregated Witness)

Bitcoin protocol upgrade that moves signatures ("witness data") outside
the TXID, fixing transaction malleability and enabling scaling.

## Satpoint

A specific satoshi inside a UTXO, as defined by Ordinal theory. Numbers
bind to satpoints during inscription.

## RPC (Remote Procedure Call)

Communication method for external software (like Numbers) to interact
with Bitcoin Core.

## Merkle Tree

A cryptographic structure that summarizes all transactions in a block.
Each transaction is hashed, paired, and re-hashed until only one hash
remains: the **Merkle root**, which is stored in the block header.

**ASCII diagram:**

                   Merkle Root
                  /          \
            HashAB            HashCD
            /    \           /    \
        HashA   HashB    HashC   HashD

    HashA = TX1 hash
    HashB = TX2 hash
    HashC = TX3 hash
    HashD = TX4 hash

If any transaction changes, the Merkle root changes, ensuring integrity
of the entire block.
