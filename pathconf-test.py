#/usr/bin/env python3

import os
import sys


print(f"Python Pathconf_names results on platform: {sys.platform}")
for key, value in sorted(os.pathconf_names.items(), key=lambda f: f[1]):
    print(f"\t{key} :  {value}")
    
print(f"\nPython Pathconf results on platform: {sys.platform}")

for i in range(0,30):
    try:
        pc_value = os.pathconf("/",i)
        print(f"\tpathconf for {i:4} is {pc_value}")
    except Exception as e:
        print(f"\tpathconf for {i:4} is Error {e}")
