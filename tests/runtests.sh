#!/bin/bash
cd "$(dirname "$0")"

# Unit tests
python regtest.py -i "../target/debug/encrusted-term-ui" czech.z3.regtest
python regtest.py -i "../target/debug/encrusted-term-ui" czech.z4.regtest
python regtest.py -i "../target/debug/encrusted-term-ui" czech.z5.regtest
python regtest.py -i "../target/debug/encrusted-term-ui" czech.z8.regtest
python regtest.py -i "../target/debug/encrusted-term-ui" praxix.z5.regtest

# Game tests
python regtest.py -i "../target/debug/encrusted-term-ui" curses.z3.regtest
python regtest.py -i "../target/debug/encrusted-term-ui" minizork.z3.regtest
