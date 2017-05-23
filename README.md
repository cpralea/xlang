# Overview
X is a toy programming language and associated compiler that I am creating as a hobby in order to learn about Rust and LLVM.

# Grammar
Refer to ```doc/grammar.txt``` for details.

At this point, the most complex programs one can write look something like this:
```
hello = "hello"
message = hello
print message
```

# Components

## xlrt Runtime
The runtime is located under ```xlrt```.

For now it only exposes three functions backing the ```print``` operation for booleans, integers and strings.

## xlc Compiler
The compiler is located under ```xlc```.

It supports the following command line parameters:
```
$ xlc --help
Usage:
    xlc [OPTIONS] XL_FILE

X language compiler. Converts X source code into LLVM IR.

positional arguments:
  XL_FILE               X source file name

optional arguments:
  -h,--help             show this help message and exit
  -v,--verbose          verbose compiler output
  --no-output           turn off compiler output so that no LL_FILE file will
                        be created
  -o,--output LL_FILE   LLVM IR output file name; defaults to FILE.ll
```

## Tools
The tools are located under ```xlc/tools```.

The ones available are:

* ```genref.py``` for generating sample compiler and program execution output for ```.xl``` files under ```tests```.
* ```xl.py``` frontend for compiling and executing ```.xl``` source files.

## Tests
The tests are located under ```xlc/tests```.

They cover compiler and program execution output (via ```xlc --verbose``` and ```xl.py --quiet```).

# Platforms
The project has been tested under 64 bit Linux and Windows.

# Prerequisites
* Rust
* Python 3
* Clang

# Build

1. Make sure cargo, python and clang are available in PATH.
2. Build the runtime and the compiler.
    ```
    $ cd xlang/xlc
    $ cargo build --all
    ```
3. Execute the test suite.
    ```
    $ cargo test --all
    ```

# Run
```
$ cd xlang/xlc
$ echo 'print "hello!"' > hello.xl
$ tools/xl.py -q hello.xl
hello!
```
