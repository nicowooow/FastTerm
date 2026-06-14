# FastTerm <3

I will create an own terminal personalizable 

## Rust Instalation 

- Install Rust on Windows

```bash
winget install Rustlang.Rustup
```
- Check versions, if you cant see the versions try it [**Solve Rust Instalation**](#solve-rust-instalation)
```bash
rustup --version
rustc --version
cargo --version
```

### Solve Rust Instalation

First find the app with and capy the path `C:\...\.cargo\bin`

```bash
Get-ChildItem "$env:USERPROFILE\.cargo\bin"
```
Add the path with 

```bash
$env:PATH += ";C:\...\.cargo\bin"
```

Try to check the versions 

```bash
rustup --version
rustc --version
cargo --version
```

## Extensions to VSCode

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Slint](https://marketplace.visualstudio.com/items?itemName=Slint.slint)


## Start Project

Starting the project with the next command, which in your current directory it will be creating a structure.
```
Your_directory
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
```
- command

```bash
cargo init
```

```bash```
