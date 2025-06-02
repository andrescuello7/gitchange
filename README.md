# Git Change

This is of project for system and config envariaments of git


## Instalación en Windows

Download ultimate version to Windows (archivo `.exe`):

- [gitchange.exe (v1.0.0)](https://github.com/andrescuello7/gitchange/blob/main/target/x86_64-pc-windows-gnu/release/gitchange.exe)

## Instalación en Linux

```bash
$ apt install gitchange
```

## Instalación en MacOs

```bash
$ brew install gitchange
```


Develop Install
-----------
- Cargo


```bash
# config in system binlocal
$ chmod +x binlocal
$ sudo mv binlocal /usr/local/bin/binlocal

# make project
$ cargo run --release
$ binlocal -d ../gitchange

# run project
$ gitchange
```


Commits
-----------
For commits add structured for easy correction and detect issues

```bash
[ADD] Added method of function for correct operation App
[IMP] or [FEAD] Implementation of new part of App

[BUG] Detection and correction of Bugs in code
[FIX] Detection and correction of fixes and future issues
[HOTFIX] Correction issue IMPORTANT!
```

**That was all, thank!** 
- **Authors: Andy**