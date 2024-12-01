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

You can also use WSL for an easy experience.

Some of the custom features included in this tool, such as the use of tips, require a modified cross channel binary. You can download that binary [here](https://mega.nz/folder/pglTVKyQ#uqVoRXCRS8Y8Vaxd0ktW7A).

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

### About the custom tip opcode

The modified CC binary has support for custom tips. This allows the game to display translation notes inserted into the script, based on whether a file called either `ALL_TIPS.txt` or `OTAKU_TIPS.txt` is present in the same folder as `cce.exe`.

Here's how this opcode will look in use, with some added context around the use site:

```yaml
  # The previous text box contents will be in this instruction. 
- !OP_45                    
  address: 0x000007C9
  opcode: 0x45
  header: [ 0xFF, 0xFF, 0x14, 0x00 ]
  sjis_bytes: [ 0x97, 0x76, 0x82, 0xB7, 0x82, 0xE9, 0x82, 0xC9, 0x83, 0x78, 0x83, 0x62, 0x83, 0x68, 0x83, 0x5E, 0x83, 0x45, 0x83, 0x93, 0x82, 0xBE, 0x81, 0x42, 0x00 ]
  size: 30
  unicode: A "bed town", essentially. # 要するにベッドタウンだ。
  # This opcode waits for user input, such as a tap of the enter button, or the mouse.
- !OP_4A
  address: 0x000007E7
  opcode: 0x4A
  arg1: 0xFFFF
  # you need to use this OP_Insert structure when adding bits into the script.
- !OP_Insert
  contents:
  - !OP_CUSTOM_TIP_77
    address: 0x000004CD
    opcode: 0x77
    # If you always want this tip to display, set to 0.
    # If this is for an obscure detail, set to 1.
    # If this is for a generally known detail, set to 2.
    # If you want to disable this tip, set to 3 or above.
    condition: 1            # In this example, this is a relatively unknown tip.
    # This number indicates how many instructions need to be skipped to not read out the tip. 
    # It should usually coincide with the number of instructions within the 
    # OP_Insert block unless you are inserting more than just this one tip. 
    skip: 4
    # This opcode is used to clear the textbox.
  - !OP_49                  
    address: 0x000005D7
    opcode: 0x49
    arg1: 0xFFFF
    arg2: 0xFFFF
    # Not sure what this does, but it's probably better to keep it around.
  - !OP_4F
    address: 0x00000584
    opcode: 0x4F
    arg1: 0x000D
    arg2: 0x0000
    # This is the opcode that actually displays text.
    # You don't need to touch the header, size or sjis_bytes bits.
  - !OP_45
    address: 0x000004CD
    opcode: 0x45
    header: [ 0xFF, 0xFF, 0x14, 0x00 ]
    sjis_bytes: []
    size: 0
    unicode: Tip - A bed town, or commuter town, is a town where there%Nare few or no businesses, just houses. People only come to%Nsuch towns to sleep or retire.
    # Always make sure there's a 4A instruction at the end, or the game will just skip the added text.
  - !OP_4A
    address: 0x00000577
    opcode: 0x4A
    arg1: 0xFFFF
  # Try to preserve the order of instructions wherever possible.
  # You will notice patterns of 4A, 6A, 49 and 4F a lot; 
  # make sure any additions you make respect this order.
- !OP_6A
  address: 0x000007EA
  opcode: 0x6A
  arg1: 0xFFFF
  arg2: 0xFFFF
- !OP_49
  address: 0x000007EF
  opcode: 0x49
  arg1: 0xFFFF
  arg2: 0xFFFF
- !OP_4F
  address: 0x000007F4
  opcode: 0x4F
  arg1: 0x000D
  arg2: 0x0000
```

