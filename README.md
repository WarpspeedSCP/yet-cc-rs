## Yeti

A tool to decode and re-encode scenario files for CROSS†CHANNEL final complete (for PC).


### Installation

This tool needs Rust, you can find instructions to install Rust here: https://rustup.rs/

Once you're done, run the following commands on your terminal to download and compile/install this tool.

```bash
git clone https://github.com/WarpspeedSCP/yet-cc-rs.git
cd yet-cc-rs
cargo install --path .
```

If you only want to build the tool without installing it, use the command below instead of the last line above:

```bash
cargo build --release
```

You can then run the command like so:
```bash
./target/release/yeti
```

These instructions are written from a Unix perspective. If your are running Windows, I suggest you install git bash from here, first: https://git-scm.com/download/win

Then, you will be able to run the commands above using the Git Bash terminal.

Some of the custom features included in this tool, such as the use of tips, require a modified cross channel binary. See [here](https://wscp.dev/404) for more details (Yes, the link is broken for now. I'll fix that soon, once I've set up everything I need.)

### Examples

- **To extract a scenario file:**

```bash
./target/release/yeti decomp --input <path/to/sn.bin> --output <output/directory>
```

This will decompress, extract and disassemble all scripts from the scenario file into the output directory you specify. Replace the parts in angular brackets with values convenient to you.

- **To recreate a scenario file:**

```bash
./target/release/yeti recomp --input <output/directory>  --output <new/sn.bin.filename> 
```

This will take all the scripts in a directory, and assemble, combine and recompress them to create a new scenario file, then put that file where you specify in `<new/sn.bin.filename>`.

- **To fix strings in a yaml script:**

```bash
./target/release/yeti fix --input <a/script/file.yaml> --output <fixed/script/file.yaml>
```

This will read the input script file (say, from where you extracted the script) and output a version with corrected spacing. This is useful when lines don't look right onscreen, or to check if you need to split text across multiple text boxes.
