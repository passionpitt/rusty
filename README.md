# Simple Rusty Project

## Overview
Simple rust application

## Prerequisites
- [install](https://doc.rust-lang.org/cargo/getting-started/installation.html) cargo & rust
- [mysql](https://www.mysql.com/)

## Setup
- database
```sh
mysql -uroot -p < setup/database.sql
```
- verify import
```sh
mysql -uroot -p -e "USE rusty;SHOW TABLES\G;"
```

## Run package
```sh
cargo run --package rusty --bin rusty
```
