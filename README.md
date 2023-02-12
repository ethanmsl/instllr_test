# instllr_test
Testing use of os commands and running installations via rust 

# ~~Build *de facto* universal for Mac:~~
~~(builds for `intel`, but works on `m1` via Rosetta)~~
```shell
rustup target install x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```
^ runs, but the virtualization seems to affect and break brew (which is, itself, version sensitive)


# run binary (despite mac security settings)
Command to run in same dir as file.
(makes file executable and removes "quarantine" extended-attribute)
```shell
ISHALLBERUN=instllr_tst
xattr -d com.apple.quarantine $ISHALLBERUN
chmod +x $ISHALLBERUN
./$ISHALLBERUN
```

# M1 Mac Paths:
```zsh
/opt/homebrew/share/zsh-autosuggestions/zsh-autosuggestions.zsh
/opt/homebrew/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
```

# Intel Mac Paths:
```zsh
/usr/local/share/zsh-autosuggestions/zsh-autosuggestions.zsh
/usr/local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
```
