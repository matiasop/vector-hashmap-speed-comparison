# vector-hashmap-speed-comparison

## Rust Results
- Array/HashMap Length: 10, vector average time: 24ns, hashmap average time: 43ns
- Array/HashMap Length: 100, vector average time: 56ns, hashmap average time: 60ns
- Array/HashMap Length: 1000, vector average time: 154ns, hashmap average time: 66ns
- Array/HashMap Length: 10000, vector average time: 762ns, hashmap average time: 73ns
- Array/HashMap Length: 100000, vector average time: 1.577µs, hashmap average time: 99ns
- Array/HashMap Length: 1000000, vector average time: 3.466µs, hashmap average time: 263ns
- Array/HashMap Length: 10000000, vector average time: 319.805µs, hashmap average time: 596ns


## Python Results
- Array/HashMap Length: 10, vector average time: 4.291534423828125e-07, hashmap average time: 1.4066696166992187e-07
- Array/HashMap Length: 100, vector average time: 5.321502685546875e-06, hashmap average time: 2.1219253540039063e-07
- Array/HashMap Length: 1000, vector average time: 4.3807029724121096e-05, hashmap average time: 5.14984130859375e-07
- Array/HashMap Length: 10000, vector average time: 0.0002840709686279297, hashmap average time: 1.049041748046875e-06
- Array/HashMap Length: 100000, vector average time: 0.0029477477073669434, hashmap average time: 2.4819374084472657e-06
- Array/HashMap Length: 1000000, vector average time: 0.029317944049835204, hashmap average time: 2.968311309814453e-06
