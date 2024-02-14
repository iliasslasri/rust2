
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

when we change the classic add to the saturating add, the program runs without any error and the result is :
```
fibo(42) = 433494437
fibo(43) = 701408733
fibo(44) = 1134903170
fibo(45) = 1836311903
fibo(46) = 2971215073
fibo(47) = 4294967295
fibo(48) = 4294967295
fibo(49) = 4294967295
fibo(50) = 4294967295
```

Using checked_add we have the following error:
```
fibo(46) = 2971215073
thread 'main' panicked at src/main.rs:17:36:
called `Option::unwrap()` on a `None` value
```
because the checked_add returns None when the result is too large to be stored in a 32-bit unsigned integer.


Displaying Only Correct Values succeded by using the checked_add function if the result is too large to be stored in a 32-bit unsigned integer, the function return None, then we break the loop, and we display:
```
fibo(45) = 1836311903
fibo(46) = 2971215073
fibo(47) = Error overflow
```
cargo clippy doesn't suggest any changes to the code. if let Some(…) = … { } doesn't do the same thing as my code using match.