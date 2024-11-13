#!/usr/bin/env python3

import sys
import re

input_file = sys.argv[1]

with open(input_file) as f:
    re_obj = re.compile(r"/p(\d{5,5})_(.+)\.rs.+\[(\d+)\. (.+)\]\(https://leetcode.com/problems/(.+)/description/\)")
    lines = f.readlines()
    ok = True
    for line in lines:
        matches = re_obj.findall(line)
        if len(matches) < 1:
            print("comment not matched", line)
            ok = False
            continue
        parts = matches[0]
        part2 = parts[2].zfill(5)
        part3 = parts[3].lower().replace(" ", "_").replace("-", "_").replace("(", "").replace(")", "")
        part4 = parts[4].lower().replace("-", "_")
        if part2 != parts[0]:
            print("problem id not match", parts)
            ok = False
        if parts[1] != part3 or parts[1] != part4:
            print("title not match", parts)
            ok = False
    if ok:
        print("ok")
    else:
        print("not ok")