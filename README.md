## Yeti

A tool to decode and re-encode scenario files for CROSS†CHANNEL final complete (for PC). Best used in conjunction with the [yeti-edit](https://github.com/WarpspeedSCP/yeti-edit) vs code extension.

### Installation

Installing the tool can be as simple as heading to the release page, downloading the latest exe file, then placing the exe file in the folder where you expect your scenario files to be.

### Building

If you want to build this tool, you'll need Rust. If you'd just like to use it, just head over to the releases for the latest exe. 

You can find instructions to install Rust here, if you need it: https://rustup.rs/

Once you're done, run the following commands on your terminal to download and compile/install this tool.

```bash
git clone https://github.com/WarpspeedSCP/yet-cc-rs.git
cd yet-cc-rs
cargo install --path .
```

Note that this will only work if the cargo install directory, usually `$HOME/.cargo/bin`, is in your `$PATH` env variable.

You might need to modify your `.bashrc` or equivalent shell init script to get this to work. 

I don't know enough about how Windows does things, so if it comes to that, consider putting the compiled `yeti.exe` file (located at `target\release\yeti.exe`) somewhere accessible, or edit the path to point there (Check this post out: https://stackoverflow.com/questions/4822400/register-an-exe-so-you-can-run-it-from-any-command-line-in-windows).

If you only want to build the tool without installing it, use the command below instead of the last line above:

```bash
cargo build --release
```

You can then run the command like so:

```bash
./target/release/yeti
```

On windows, you'd run it like this instead:

```batch
target\release\yeti.exe
```

These instructions are written from a Unix perspective. If your are running Windows, I suggest you install git bash from here, first: https://git-scm.com/download/win

Then, you will be able to run the commands above using the Git Bash terminal.

You can also use WSL for an easy experience.

### Custom Cross Channel binary

The custom features included in this tool, such as the use of tips, require a modified cross channel binary. That is to say, **If you run a modified script that uses custom tips with normal cross channel, the game will crash.** You can download that binary [here](https://mega.nz/folder/pglTVKyQ#uqVoRXCRS8Y8Vaxd0ktW7A).

### Examples

**NOTE**: If you're on windows, replace `yeti` with `./yeti.exe`. (assuming the yeti executable is in the current directory)

- **To extract a scenario file:**

```bash
yeti unpack --input <path/to/sn.bin> --output <output/directory> --textdir <script text directory>
```

This will decompress, extract and disassemble all scripts from the scenario file into the output directory you specify. Replace the parts in angular brackets with values convenient to you. 

The text script directory is the place where the text version of the scripts will be written. It's recommended to only edit the text scripts, as it is far easier to edit a text file than a yaml file.

- **To recreate a scenario file:**

```bash
yeti pack --input <input/directory>  --output <new/sn.bin.filename> --textdir <script text directory>
```

This will take all the scripts in the input directory, assemble, combine and recompress them to create a new scenario file, then put that file where you specify in `<new/sn.bin.filename>`.

- **To check/fix strings in a yaml script:**

```bash
yeti fix --input <a/script/file.yaml> --output <fixed/script/file.yaml>
```

This will read the input script file (say, from where you extracted the script) and output a version with corrected spacing. This is useful when lines don't look right onscreen, or to check if you need to split text across multiple text boxes.

If you only need to check if you need to split a line across multiple textboxes, don't specify the output parameter. 

## FAQ

### How do I insert new lines into the script?

You use something called `OP_Insert` for this.

This is what it looks like in yaml, generally:

```yaml
...
- !OP_Insert
  contents:
    - !Instr1
      ...
    - !Instr2
      ...
    ...
...
```

Here's an example usage:

```yaml
- !OP_TEXTBOX_DISPLAY
  address: 0x00000FC7
  opcode: 0x45
  header: [ 0xFF, 0xFF, 0x2C, 0x00 ]
  sjis_bytes: [ ... ] # Don't change anything here.
  size: 118
  unicode: Sakuraba's family was the exact opposite, and were quite%Ngentle in comparison. His father was a decorated veteran # 桜庭一族は正反対におっとりとした一族で、父親は盲腸の痛みに気づかず死の一歩手前まで旅だったという武勲の持ち主だ。
- !OP_Insert
  contents:
  - !OP_WAIT
    address: 0x000005CF
    opcode: 0x4A
    arg1: 0xFFFF
  - !OP_CLEAR_SCREEN
    address: 0x000005D7
    opcode: 0x49
    arg1: 0xFFFF
    arg2: 0xFFFF
  - !OP_TEXTBOX_DISPLAY
    address: 0x000004CD
    opcode: 0x45
    header: [ 0xFF, 0xFF, 0x2C, 0x00 ]
    sjis_bytes: []
    size: 0
    unicode: who'd very narrowly skirted death setting out as he did,%Nmindless of even the pain of an infected appendix. # 桜庭一族は正反対におっとりとした一族で、父親は盲腸の痛みに気づかず死の一歩手前まで旅だったという武勲の持ち主だ。
- !OP_WAIT
  address: 0x0000103D
  opcode: 0x4A
  arg1: 0xFFFF
```

Here are a few things to keep in mind when inserting opcodes:

1. Make sure your text isn't longer than 180 characters per textbox. The `yeti fix` command can be used to check if your line exceeds that number.
2. Any text instructions that are within an insert structure should have their `sjis_bytes` and `size` fields set to `[]` and `0` respectively. Not doing so will give you some annoying problems.
3. Don't attempt to modify things like opcode or header values, things could blow up.
4. Try to preserve the order of instructions wherever possible.
   - You will notice patterns of 4A, 6A, 49 and 4F a lot;
   - make sure any additions you make respect this order.
### How do I insert a new line into whatever I write in a text opcode?

Use `%N` to add a line break in the text.

```yaml
  - !OP_TEXTBOX_DISPLAY
    address: 0x000004CD
    opcode: 0x45
    header: [ 0xFF, 0xFF, 0x14, 0x00 ]
    sjis_bytes: []
    size: 0
    unicode: A%NB%NC # This will show up as A, B and C on 3 separate lines.
```

### How do I use the custom tip opcode?

The modified CC binary has support for custom tips. This allows the game to display translation notes inserted into the script, based on whether a file called either `ALL_TIPS.txt` or `OTAKU_TIPS.txt` is present in the same folder as `cce.exe`.

Here's how this opcode will look in use, with some added context around the use site:

```yaml
  # The previous text box contents will be in this instruction. 
- !OP_TEXTBOX_DISPLAY                    
  address: 0x000007C9
  opcode: 0x45
  header: [ 0xFF, 0xFF, 0x14, 0x00 ]
  sjis_bytes: [ 0x97, 0x76, 0x82, 0xB7, 0x82, 0xE9, 0x82, 0xC9, 0x83, 0x78, 0x83, 0x62, 0x83, 0x68, 0x83, 0x5E, 0x83, 0x45, 0x83, 0x93, 0x82, 0xBE, 0x81, 0x42, 0x00 ]
  size: 30
  unicode: A "bed town", essentially. # 要するにベッドタウンだ。
  # This opcode waits for user input, such as a tap of the enter button, or the mouse.
- !OP_WAIT
  address: 0x000007E7
  opcode: 0x4A
  arg1: 0xFFFF
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
    # To display this tip, the game needs to run 4 instructions after the custom tip instruction.
    # So, you write 4 here.
    skip: 4
    # This opcode is used to clear the textbox.
  - !OP_CLEAR_SCREEN                  
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
  - !OP_TEXTBOX_DISPLAY
    address: 0x000004CD
    opcode: 0x45
    header: [ 0xFF, 0xFF, 0x14, 0x00 ]
    sjis_bytes: []
    size: 0
    unicode: Tip - A bed town, or commuter town, is a town where there%Nare few or no businesses, just houses. People only come to%Nsuch towns to sleep or retire.
    # Always make sure there's a 4A instruction at the end, or the game will just skip the added text.
  - !OP_WAIT
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
- !OP_CLEAR_SCREEN
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

