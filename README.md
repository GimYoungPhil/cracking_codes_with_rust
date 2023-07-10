```sh
/cracking_codes_with_rust
    cargo.toml
    /src
        main.rs
```


```sh
> cargo run -- 13 encoding plain.txt
```

key: number
mode: encoding | decoding
file_path:

```sh
> $Env:IGNORE_DESC=1; cargo run -- 13 encoding plain.txt
```

```sh
> Remove-Item Env:IGNORE_DESC
```



```sh
> cargo run -- 13 encoding plain.txt > output.txt
```
