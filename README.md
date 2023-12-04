## Convert Wireshark hexdump to Rust u8 arry

```
cargo install --path .
# With wireshark hexdump in clipbroad

xclip -o -selection c | hex_to_rust | xclip -selection c

# The rust u8 array is stored in your clipbroad
```
