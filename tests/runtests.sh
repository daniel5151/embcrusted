#!/bin/bash
cd "$(dirname "$0")"

# Unit tests
python regtest.py -i "../target/debug/encrusted-ui" czech.z3.regtest
python regtest.py -i "../target/debug/encrusted-ui" czech.z4.regtest
python regtest.py -i "../target/debug/encrusted-ui" czech.z5.regtest
python regtest.py -i "../target/debug/encrusted-ui" czech.z8.regtest
python regtest.py -i "../target/debug/encrusted-ui" praxix.z5.regtest
