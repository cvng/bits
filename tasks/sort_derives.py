#!/usr/bin/python3

"""Sort derives in Rust code to be consistent. https://github.com/rust-lang/rustfmt/issues/4112"""

import os
import fileinput
import re

a = re.compile(r"^#\[derive\((\s*[a-zA-Z0-9_]+\s*,)*(\s*[a-zA-Z0-9_]+\s*)\)]$")

special = ["Copy", "Clone", "Eq", "PartialEq", "Ord", "PartialOrd", "Hash", "Debug", "Display", "Default", "Serialize", "Deserialize"]

special = {x: i for i, x in enumerate(special)}

for root, dirs, files in os.walk("."):
    if "target" in root:
        continue
    for name in files:
        path = root + os.sep + name

        if name.endswith(".rs"):
            for line in fileinput.input(path, inplace=True):
                if a.match(line):
                    derives = line[9:-3]
                    derives = [(special.get(x.strip(), 100), x.strip()) for x in derives.split(",")]
                    derives.sort()
                    line = "#[derive(" + ", ".join(x[1] for x in derives) + ")]"
                    print(line)
                else:
                    print(line, end="")
