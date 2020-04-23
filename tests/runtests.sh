#!/bin/bash
cd "$(dirname "$0")"

# Unit tests
python regtest.py -i "../target/debug/term-ui-no-std" czech.z3.regtest
python regtest.py -i "../target/debug/term-ui-no-std" czech.z4.regtest
python regtest.py -i "../target/debug/term-ui-no-std" czech.z5.regtest
python regtest.py -i "../target/debug/term-ui-no-std" czech.z8.regtest
python regtest.py -i "../target/debug/term-ui-no-std" praxix.z5.regtest

# Game tests
python regtest.py -i "../target/debug/term-ui-no-std" curses.z3.regtest
python regtest.py -i "../target/debug/term-ui-no-std" minizork.z3.regtest
