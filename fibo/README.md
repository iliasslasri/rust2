
In the debug mode we have :

```
fibo(42) = 267914296
Time elapsed: 5.052903101s
```

Where in the realease mode we have :
```
fibo(42) = 267914296
Time elapsed: 1.517708928s
```

###### The release mode is 3.33 times faster than the debug mode.


When we change the max value to 50, the program stops with the following error:
```
fibo(47) = 2971215073
thread 'main' panicked at src/main.rs:20:17:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
This is because the next number in the sequence is too large to be stored in a 32-bit unsigned integer, so it overflows.