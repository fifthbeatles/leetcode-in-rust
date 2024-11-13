#!/bin/bash

cd $(dirname $0)
tmp_file=target/url.txt
git grep leetcode.com src/problems/ > ${tmp_file}
python3 check_url.py ${tmp_file}
rm $tmp_file
