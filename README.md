# go-wc

Rust implementation of linux cli command wc. it counts number of words, lines, bytes, and characters in a file.


### Features
- Counts words, lines, and characters

### Getting Started

Follow the steps below to get started:

1. Clone the repository using Git:

   ```bash
   git clone https://github.com/zeenfaizpy/wc-rs
   ```

2. Change to the project directory:

   ```bash
   cd wc-rs
   ```

3. Build the binary:

   ```bash
   cargo build --release
   ```

### Usage

Move the binary wc-rs ( in target/release) to /usr/bin and
run the following or type the full path of the binary

```bash
wc-rs [filename]
````
