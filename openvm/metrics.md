| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  269.81 |  57.31 |
| all |  266.72 |  41.62 |
| baseline |  3.09 |  3.09 |


| all |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  33,340.25 |  266,722 |  41,624 |  26,263 |
| `main_cells_used     ` |  361,597,803.50 |  2,892,782,428 |  385,213,140 |  198,317,403 |
| `total_cycles        ` |  8,315,344.88 |  66,522,759 |  8,855,882 |  4,558,451 |
| `execute_time_ms     ` |  1,839 |  14,712 |  2,117 |  999 |
| `trace_gen_time_ms   ` |  3,154.13 |  25,233 |  3,512 |  1,805 |
| `stark_prove_excluding_trace_time_ms` |  28,347.13 |  226,777 |  35,995 |  23,459 |
| `main_trace_commit_time_ms` |  5,849.63 |  46,797 |  7,101 |  4,742 |
| `generate_perm_trace_time_ms` |  3,737 |  29,896 |  4,564 |  2,954 |
| `perm_trace_commit_time_ms` |  6,980.13 |  55,841 |  8,541 |  6,463 |
| `quotient_poly_compute_time_ms` |  4,068.63 |  32,549 |  5,431 |  2,993 |
| `quotient_poly_commit_time_ms` |  1,902.13 |  15,217 |  2,186 |  1,764 |
| `pcs_opening_time_ms ` |  5,792.38 |  46,339 |  8,142 |  4,338 |

| baseline |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,089 |  3,089 |  3,089 |  3,089 |
| `main_cells_used     ` |  11,444,124 |  11,444,124 |  11,444,124 |  11,444,124 |
| `total_cycles        ` |  232,762 |  232,762 |  232,762 |  232,762 |
| `execute_time_ms     ` |  29 |  29 |  29 |  29 |
| `trace_gen_time_ms   ` |  169 |  169 |  169 |  169 |
| `stark_prove_excluding_trace_time_ms` |  2,891 |  2,891 |  2,891 |  2,891 |
| `main_trace_commit_time_ms` |  532 |  532 |  532 |  532 |
| `generate_perm_trace_time_ms` |  121 |  121 |  121 |  121 |
| `perm_trace_commit_time_ms` |  532 |  532 |  532 |  532 |
| `quotient_poly_compute_time_ms` |  210 |  210 |  210 |  210 |
| `quotient_poly_commit_time_ms` |  434 |  434 |  434 |  434 |
| `pcs_opening_time_ms ` |  1,055 |  1,055 |  1,055 |  1,055 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | fri.log_blowup | commit_exe_time_ms |
| --- | --- | --- | --- | --- |
| all | 8 | 1,083 | 1 | 9 | 
| baseline | 1 | 1,139 | 1 | 9 | 

| group | air_name | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| all | AccessAdapterAir<16> | 2 | 5 | 12 | 
| all | AccessAdapterAir<2> | 2 | 5 | 12 | 
| all | AccessAdapterAir<32> | 2 | 5 | 12 | 
| all | AccessAdapterAir<4> | 2 | 5 | 12 | 
| all | AccessAdapterAir<8> | 2 | 5 | 12 | 
| all | BitwiseOperationLookupAir<8> | 2 | 2 | 4 | 
| all | KeccakVmAir | 2 | 321 | 4,513 | 
| all | MemoryMerkleAir<8> | 2 | 4 | 39 | 
| all | PersistentBoundaryAir<8> | 2 | 3 | 7 | 
| all | PhantomAir | 2 | 3 | 5 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 1 | 286 | 
| all | ProgramAir | 1 | 1 | 4 | 
| all | RangeTupleCheckerAir<2> | 1 | 1 | 4 | 
| all | Rv32HintStoreAir | 2 | 18 | 28 | 
| all | Sha256VmAir | 2 | 50 | 663 | 
| all | VariableRangeCheckerAir | 1 | 1 | 4 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 20 | 37 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 18 | 40 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 24 | 91 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 11 | 20 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 13 | 35 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 10 | 18 | 
| all | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 2 | 61 | 126 | 
| all | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 2 | 31 | 129 | 
| all | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 2 | 61 | 57 | 
| all | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 2 | 79 | 2,161 | 
| all | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 2 | 20 | 55 | 
| all | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 2 | 22 | 126 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 2 | 25 | 225 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 16 | 20 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 18 | 33 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 17 | 40 | 
| all | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 2 | 25 | 84 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 24 | 31 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 19 | 19 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 12 | 14 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 415 | 480 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 2 | 158 | 190 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 428 | 457 | 
| all | VmConnectorAir | 2 | 5 | 11 | 
| baseline | AccessAdapterAir<16> | 2 | 5 | 12 | 
| baseline | AccessAdapterAir<2> | 2 | 5 | 12 | 
| baseline | AccessAdapterAir<32> | 2 | 5 | 12 | 
| baseline | AccessAdapterAir<4> | 2 | 5 | 12 | 
| baseline | AccessAdapterAir<8> | 2 | 5 | 12 | 
| baseline | BitwiseOperationLookupAir<8> | 2 | 2 | 4 | 
| baseline | KeccakVmAir | 2 | 321 | 4,513 | 
| baseline | MemoryMerkleAir<8> | 2 | 4 | 39 | 
| baseline | PersistentBoundaryAir<8> | 2 | 3 | 7 | 
| baseline | PhantomAir | 2 | 3 | 5 | 
| baseline | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 1 | 286 | 
| baseline | ProgramAir | 1 | 1 | 4 | 
| baseline | RangeTupleCheckerAir<2> | 1 | 1 | 4 | 
| baseline | Rv32HintStoreAir | 2 | 18 | 28 | 
| baseline | Sha256VmAir | 2 | 50 | 663 | 
| baseline | VariableRangeCheckerAir | 1 | 1 | 4 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 20 | 37 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 18 | 40 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 24 | 91 | 
| baseline | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 11 | 20 | 
| baseline | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 13 | 35 | 
| baseline | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 10 | 18 | 
| baseline | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 2 | 61 | 126 | 
| baseline | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 2 | 31 | 129 | 
| baseline | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 2 | 61 | 57 | 
| baseline | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 2 | 79 | 2,161 | 
| baseline | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 2 | 20 | 55 | 
| baseline | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 2 | 22 | 126 | 
| baseline | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 2 | 25 | 225 | 
| baseline | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 16 | 20 | 
| baseline | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 18 | 33 | 
| baseline | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 17 | 40 | 
| baseline | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 2 | 25 | 84 | 
| baseline | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 24 | 31 | 
| baseline | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 19 | 19 | 
| baseline | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 12 | 14 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 415 | 480 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 2 | 158 | 190 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 428 | 457 | 
| baseline | VmConnectorAir | 2 | 5 | 11 | 

| group | air_name | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| all | AccessAdapterAir<16> | 0 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 1 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 2 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 3 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 4 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 5 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 6 | 65,536 |  | 16 | 25 | 2,686,976 | 
| all | AccessAdapterAir<16> | 7 | 32,768 |  | 16 | 25 | 1,343,488 | 
| all | AccessAdapterAir<2> | 0 | 128 |  | 16 | 11 | 3,456 | 
| all | AccessAdapterAir<32> | 0 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 1 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 2 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 3 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 4 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 5 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 6 | 32,768 |  | 16 | 41 | 1,867,776 | 
| all | AccessAdapterAir<32> | 7 | 16,384 |  | 16 | 41 | 933,888 | 
| all | AccessAdapterAir<4> | 0 | 64 |  | 16 | 13 | 1,856 | 
| all | AccessAdapterAir<8> | 0 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 1 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 2 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 3 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 4 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 5 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 6 | 262,144 |  | 16 | 17 | 8,650,752 | 
| all | AccessAdapterAir<8> | 7 | 131,072 |  | 16 | 17 | 4,325,376 | 
| all | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | BitwiseOperationLookupAir<8> | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| all | KeccakVmAir | 0 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 1 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 2 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 3 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 4 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 5 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 6 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | KeccakVmAir | 7 | 1 |  | 1,056 | 3,163 | 4,219 | 
| all | MemoryMerkleAir<8> | 0 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 1 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 2 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 3 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 4 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 5 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 6 | 131,072 |  | 16 | 32 | 6,291,456 | 
| all | MemoryMerkleAir<8> | 7 | 65,536 |  | 16 | 32 | 3,145,728 | 
| all | PersistentBoundaryAir<8> | 0 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 1 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 2 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 3 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 4 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 5 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 6 | 131,072 |  | 12 | 20 | 4,194,304 | 
| all | PersistentBoundaryAir<8> | 7 | 65,536 |  | 12 | 20 | 2,097,152 | 
| all | PhantomAir | 0 | 4 |  | 12 | 6 | 72 | 
| all | PhantomAir | 1 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 2 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 3 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 4 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 5 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 6 | 1 |  | 12 | 6 | 18 | 
| all | PhantomAir | 7 | 1 |  | 12 | 6 | 18 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 1 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 3 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 4 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 5 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 6 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 7 | 4,096 |  | 8 | 300 | 1,261,568 | 
| all | ProgramAir | 0 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 1 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 2 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 3 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 4 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 5 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 6 | 32,768 |  | 8 | 10 | 589,824 | 
| all | ProgramAir | 7 | 32,768 |  | 8 | 10 | 589,824 | 
| all | RangeTupleCheckerAir<2> | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | RangeTupleCheckerAir<2> | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| all | Rv32HintStoreAir | 0 | 64 |  | 44 | 32 | 4,864 | 
| all | Sha256VmAir | 0 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 1 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 2 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 3 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 4 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 5 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 6 | 32,768 |  | 108 | 470 | 18,939,904 | 
| all | Sha256VmAir | 7 | 16,384 |  | 108 | 470 | 9,469,952 | 
| all | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VariableRangeCheckerAir | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 7 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | 131,072 |  | 40 | 37 | 10,092,544 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 7 | 65,536 |  | 40 | 37 | 5,046,272 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 1 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 3 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 4 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 5 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 6 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| all | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 7 | 524,288 |  | 52 | 53 | 55,050,240 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | 524,288 |  | 28 | 26 | 28,311,552 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 7 | 262,144 |  | 28 | 26 | 14,155,776 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 1 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 3 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 4 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 5 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 6 | 262,144 |  | 32 | 32 | 16,777,216 | 
| all | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 7 | 131,072 |  | 32 | 32 | 8,388,608 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | 131,072 |  | 28 | 18 | 6,029,312 | 
| all | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 7 | 65,536 |  | 28 | 18 | 3,014,656 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 1 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 2 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 3 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 4 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 5 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 6 | 65,536 |  | 56 | 166 | 14,548,992 | 
| all | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 7 | 32,768 |  | 56 | 166 | 7,274,496 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 1 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 3 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 4 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 5 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 6 | 524,288 |  | 36 | 28 | 33,554,432 | 
| all | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 7 | 262,144 |  | 36 | 28 | 16,777,216 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 1 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 3 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 4 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 5 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 6 | 131,072 |  | 52 | 36 | 11,534,336 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 7 | 65,536 |  | 52 | 36 | 5,767,168 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 1 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 3 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 4 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 5 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 6 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| all | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 7 | 2,097,152 |  | 52 | 41 | 195,035,136 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 1 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 3 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 4 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 5 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 6 | 1,024 |  | 72 | 39 | 113,664 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 7 | 512 |  | 72 | 39 | 56,832 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 1 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 3 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 4 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 5 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 6 | 262,144 |  | 52 | 31 | 21,757,952 | 
| all | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 7 | 131,072 |  | 52 | 31 | 10,878,976 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 1 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 3 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 4 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 5 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 6 | 262,144 |  | 28 | 20 | 12,582,912 | 
| all | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 7 | 131,072 |  | 28 | 20 | 6,291,456 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 1 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 3 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 4 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 5 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 6 | 65,536 |  | 836 | 547 | 90,636,288 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 7 | 32,768 |  | 836 | 547 | 45,318,144 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 0 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 1 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 2 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 3 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 4 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 5 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 6 | 1,024 |  | 320 | 263 | 596,992 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 7 | 512 |  | 320 | 263 | 298,496 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 1 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 3 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 4 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 5 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 6 | 16,384 |  | 860 | 625 | 24,330,240 | 
| all | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 7 | 8,192 |  | 860 | 625 | 12,165,120 | 
| all | VmConnectorAir | 0 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 1 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 2 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 3 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 4 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 5 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 6 | 2 | 1 | 16 | 5 | 42 | 
| all | VmConnectorAir | 7 | 2 | 1 | 16 | 5 | 42 | 
| baseline | AccessAdapterAir<16> | 0 | 512 |  | 16 | 25 | 20,992 | 
| baseline | AccessAdapterAir<2> | 0 | 128 |  | 16 | 11 | 3,456 | 
| baseline | AccessAdapterAir<32> | 0 | 256 |  | 16 | 41 | 14,592 | 
| baseline | AccessAdapterAir<4> | 0 | 64 |  | 16 | 13 | 1,856 | 
| baseline | AccessAdapterAir<8> | 0 | 32,768 |  | 16 | 17 | 1,081,344 | 
| baseline | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| baseline | KeccakVmAir | 0 | 1 |  | 1,056 | 3,163 | 4,219 | 
| baseline | MemoryMerkleAir<8> | 0 | 32,768 |  | 16 | 32 | 1,572,864 | 
| baseline | PersistentBoundaryAir<8> | 0 | 32,768 |  | 12 | 20 | 1,048,576 | 
| baseline | PhantomAir | 0 | 4 |  | 12 | 6 | 72 | 
| baseline | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 2,048 |  | 8 | 300 | 630,784 | 
| baseline | ProgramAir | 0 | 32,768 |  | 8 | 10 | 589,824 | 
| baseline | RangeTupleCheckerAir<2> | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| baseline | Rv32HintStoreAir | 0 | 64 |  | 44 | 32 | 4,864 | 
| baseline | Sha256VmAir | 0 | 256 |  | 108 | 470 | 147,968 | 
| baseline | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 131,072 |  | 52 | 36 | 11,534,336 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 4,096 |  | 40 | 37 | 315,392 | 
| baseline | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 8,192 |  | 52 | 53 | 860,160 | 
| baseline | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 16,384 |  | 28 | 26 | 884,736 | 
| baseline | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 16,384 |  | 32 | 32 | 1,048,576 | 
| baseline | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 8,192 |  | 28 | 18 | 376,832 | 
| baseline | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | 512 |  | 56 | 166 | 113,664 | 
| baseline | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 16,384 |  | 36 | 28 | 1,048,576 | 
| baseline | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | 2,048 |  | 52 | 36 | 180,224 | 
| baseline | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 131,072 |  | 52 | 41 | 12,189,696 | 
| baseline | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | 8 |  | 72 | 39 | 888 | 
| baseline | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | 2,048 |  | 52 | 31 | 169,984 | 
| baseline | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 8,192 |  | 28 | 20 | 393,216 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 256 |  | 836 | 547 | 354,048 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 0 | 8 |  | 320 | 263 | 4,664 | 
| baseline | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 128 |  | 860 | 625 | 190,080 | 
| baseline | VmConnectorAir | 0 | 2 | 1 | 16 | 5 | 42 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| all | 0 | 3,309 | 33,024 | 8,855,882 | 1,206,722,221 | 27,748 | 3,945 | 1,939 | 6,463 | 5,442 | 6,106 | 385,118,322 | 3,839 | 1,967 | 
| all | 1 | 3,282 | 30,831 | 8,852,751 | 1,206,711,991 | 25,633 | 3,868 | 1,782 | 6,472 | 4,480 | 5,605 | 384,998,255 | 3,407 | 1,916 | 
| all | 2 | 3,218 | 33,700 | 8,849,317 | 1,206,711,991 | 28,586 | 3,976 | 1,827 | 6,753 | 6,776 | 5,639 | 384,739,096 | 3,601 | 1,896 | 
| all | 3 | 3,290 | 34,822 | 8,849,216 | 1,206,711,991 | 29,564 | 4,454 | 1,995 | 7,996 | 4,879 | 6,030 | 384,732,836 | 4,193 | 1,968 | 
| all | 4 | 3,380 | 33,028 | 8,849,115 | 1,206,711,991 | 27,718 | 3,992 | 1,764 | 6,569 | 5,964 | 5,835 | 384,734,779 | 3,576 | 1,930 | 
| all | 5 | 3,437 | 33,430 | 8,852,953 | 1,206,711,991 | 28,074 | 3,890 | 1,792 | 6,559 | 6,318 | 5,739 | 384,928,597 | 3,762 | 1,919 | 
| all | 6 | 3,512 | 41,624 | 8,855,074 | 1,206,711,991 | 35,995 | 5,431 | 2,186 | 8,541 | 8,142 | 7,101 | 385,213,140 | 4,564 | 2,117 | 
| all | 7 | 1,805 | 26,263 | 4,558,451 | 799,777,719 | 23,459 | 2,993 | 1,932 | 6,488 | 4,338 | 4,742 | 198,317,403 | 2,954 | 999 | 
| baseline | 0 | 169 | 3,089 | 232,762 | 56,677,113 | 2,891 | 210 | 434 | 532 | 1,055 | 532 | 11,444,124 | 121 | 29 | 

| group | segment | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| all | 0 | 0 | 23,696,526 | 2,013,265,921 | 
| all | 0 | 1 | 69,100,570 | 2,013,265,921 | 
| all | 0 | 2 | 11,848,263 | 2,013,265,921 | 
| all | 0 | 3 | 116,599,134 | 2,013,265,921 | 
| all | 0 | 4 | 524,288 | 2,013,265,921 | 
| all | 0 | 5 | 262,144 | 2,013,265,921 | 
| all | 0 | 6 | 29,364,554 | 2,013,265,921 | 
| all | 0 | 7 | 65,536 | 2,013,265,921 | 
| all | 0 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 0 | 9 | 255,045,015 | 2,013,265,921 | 
| all | 1 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 1 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 1 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 1 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 1 | 4 | 524,288 | 2,013,265,921 | 
| all | 1 | 5 | 262,144 | 2,013,265,921 | 
| all | 1 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 1 | 7 | 65,536 | 2,013,265,921 | 
| all | 1 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 1 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 2 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 2 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 2 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 2 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 2 | 4 | 524,288 | 2,013,265,921 | 
| all | 2 | 5 | 262,144 | 2,013,265,921 | 
| all | 2 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 2 | 7 | 65,536 | 2,013,265,921 | 
| all | 2 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 2 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 3 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 3 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 3 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 3 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 3 | 4 | 524,288 | 2,013,265,921 | 
| all | 3 | 5 | 262,144 | 2,013,265,921 | 
| all | 3 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 3 | 7 | 65,536 | 2,013,265,921 | 
| all | 3 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 3 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 4 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 4 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 4 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 4 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 4 | 4 | 524,288 | 2,013,265,921 | 
| all | 4 | 5 | 262,144 | 2,013,265,921 | 
| all | 4 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 4 | 7 | 65,536 | 2,013,265,921 | 
| all | 4 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 4 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 5 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 5 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 5 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 5 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 5 | 4 | 524,288 | 2,013,265,921 | 
| all | 5 | 5 | 262,144 | 2,013,265,921 | 
| all | 5 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 5 | 7 | 65,536 | 2,013,265,921 | 
| all | 5 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 5 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 6 | 0 | 23,696,392 | 2,013,265,921 | 
| all | 6 | 1 | 69,099,610 | 2,013,265,921 | 
| all | 6 | 2 | 11,848,196 | 2,013,265,921 | 
| all | 6 | 3 | 116,598,366 | 2,013,265,921 | 
| all | 6 | 4 | 524,288 | 2,013,265,921 | 
| all | 6 | 5 | 262,144 | 2,013,265,921 | 
| all | 6 | 6 | 29,364,362 | 2,013,265,921 | 
| all | 6 | 7 | 65,536 | 2,013,265,921 | 
| all | 6 | 8 | 1,056,768 | 2,013,265,921 | 
| all | 6 | 9 | 255,042,894 | 2,013,265,921 | 
| all | 7 | 0 | 16,042,504 | 2,013,265,921 | 
| all | 7 | 1 | 47,132,762 | 2,013,265,921 | 
| all | 7 | 2 | 8,021,252 | 2,013,265,921 | 
| all | 7 | 3 | 70,882,142 | 2,013,265,921 | 
| all | 7 | 4 | 262,144 | 2,013,265,921 | 
| all | 7 | 5 | 131,072 | 2,013,265,921 | 
| all | 7 | 6 | 25,168,010 | 2,013,265,921 | 
| all | 7 | 7 | 32,768 | 2,013,265,921 | 
| all | 7 | 8 | 528,384 | 2,013,265,921 | 
| all | 7 | 9 | 170,728,270 | 2,013,265,921 | 
| baseline | 0 | 0 | 690,614 | 2,013,265,921 | 
| baseline | 0 | 1 | 2,048,218 | 2,013,265,921 | 
| baseline | 0 | 2 | 345,307 | 2,013,265,921 | 
| baseline | 0 | 3 | 2,506,410 | 2,013,265,921 | 
| baseline | 0 | 4 | 131,072 | 2,013,265,921 | 
| baseline | 0 | 5 | 65,536 | 2,013,265,921 | 
| baseline | 0 | 6 | 823,402 | 2,013,265,921 | 
| baseline | 0 | 7 | 512 | 2,013,265,921 | 
| baseline | 0 | 8 | 8,256 | 2,013,265,921 | 
| baseline | 0 | 9 | 9,144,511 | 2,013,265,921 | 

</details>

