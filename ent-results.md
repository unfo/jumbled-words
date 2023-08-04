# Ent results

## Time-based entropy
```
\`-->  cargo run --release 100 > output
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/jumbled-words 100`

\`-->  ../ent/src/ent output
Entropy = 7.999993 bits per byte.

Optimum compression would reduce the size
of this 104857600 byte file by 0 percent.

Chi square distribution for 104857600 samples is 981.65, and randomly
would exceed this value less than 0.01 percent of the times.

Arithmetic mean value of data bytes is 127.5050 (127.5 = random).
Monte Carlo value for Pi is 3.142379957 (error 0.03 percent).
Serial correlation coefficient is -0.000018 (totally uncorrelated = 0.0).
```

## Byte and bit shuffling

First draft:

```
./ent output/1m_shuf
Entropy = 3.046355 bits per byte.

Optimum compression would reduce the size
of this 4718592 byte file by 61 percent.

Chi square distribution for 4718592 samples is 269061736.89, and randomly
would exceed this value less than 0.01 percent of the times.

Arithmetic mean value of data bytes is 98.9861 (127.5 = random).
Monte Carlo value for Pi is 3.125000000 (error 0.53 percent).
Serial correlation coefficient is 0.877942 (totally uncorrelated = 0.0).
```