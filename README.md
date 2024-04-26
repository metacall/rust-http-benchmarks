# Web frameworks benchmarking
## Specs

```
OS: Arch Linux x86_64 
Kernel: 6.6.16-1-lts 
CPU: 13th Gen Intel i5-13500H (16) @ 4.7GHz
GPU: Intel Raptor Lake-P [UHD Graphics] 
Memory: 7569MiB
Storage: 512GB SSD
```

## Results
> **Warning**  
>  results maybe not accurate, do benchmarks by yourself 

run this command
```terminal
$ ./benchmark.sh
```
**Actix**
```
Running 3s test @ http://127.0.0.1:8080
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   382.82us    1.84ms  31.47ms   97.33%
    Req/Sec     6.73k     2.85k   13.08k    67.74%
  20734 requests in 3.10s, 4.90MB read
Requests/sec:   6688.46
Transfer/sec:      1.58MB
```

**Warp**
```
Running 3s test @ http://127.0.0.1:8081
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   285.13us    1.43ms  26.52ms   97.75%
    Req/Sec     8.43k     2.76k   13.42k    67.74%
  26012 requests in 3.10s, 6.15MB read
Requests/sec:   8392.66
Transfer/sec:      1.98MB


```
***Axum***
```
RRunning 3s test @ http://127.0.0.1:8082
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   176.40us  765.72us  15.90ms   97.62%
    Req/Sec    11.83k     2.44k   13.80k    83.87%
  36477 requests in 3.10s, 8.63MB read
Requests/sec:  11768.49
Transfer/sec:      2.78MB
```

