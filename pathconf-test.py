#/usr/bin/env python3

import os
import sys

print(f"Platform is {sys.platform}")

print("Pathconf_names results:")
for key, value in os.pathconf_names.items():
    print(f" {key} :  {value}")
    

for i in range(0,30):
    try:
        pc_value = os.pathconf("/",i)
        print(f"pathconf for {i} is {pc_value}")
    except Exception as e:
        print(f"pathconf for {i} is Error {e}")
