# zkVM Cost Result

[guest](./program/src/main.rs) program runs 1000 times with or without decryption.

with decryption:
- total cells: 9,246,771,886

without decryption:
- total cells: 56,677,113

Each decryption costs 9,190,094.773 cells.

According to the current zkVM cost model, 1 Gas ≈ 5241.67 cells.
Thus, each decryption costs approximately equivalent an EVM operation of 1753.28 Gas.
