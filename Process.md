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

Install `Desarrollo para el escritorio con C++ / Desktop development with C++` from Visual Studio with:
-  (MSVC v14x - VS C++ x64/x86 build tools)

- SDK de Windows (Windows 11 SDK o Windows 10 SDK)
- C++ CMake tools para Windows
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
- command to start project

```bash
cargo init
```
- command to run project

```bash
cargo run
```



```bash```
