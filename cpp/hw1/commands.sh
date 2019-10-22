#! /bin/sh
wc -l ./data/data.dat
grep -E '(dalor)|(dolor)' ./data/data.dat | wc -l
wc -w ./data/data.dat
cat data/data.dat| tr ' ' '\n' | grep -E '^mol.*' | wc -l
ls -l data/test_folder/*.txt | wc -l
