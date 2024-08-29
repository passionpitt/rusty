# Rusty Project

## Overview
Simple rust application

## Prerequisites
- [install](https://doc.rust-lang.org/cargo/getting-started/installation.html) cargo & rust

## Commands
- Start API Service
  ```sh
  make up
  ```
- Shut Down API Service
  ```sh
  make down
  ```
- Build API Service docker image
  ```sh
  make docker-build
  ```
- Build API Service local image
  ```sh
  make local-build
  ```
- Setup DB (note: password is `password`)
  ```sh
  make setup-db
  ```
