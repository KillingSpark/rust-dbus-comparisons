#! /bin/python3

import os

stream = os.popen('bash -c "cargo bench | grep time"')
#stream = os.popen('bash -c "cargo bench rustbus | grep time"')
output = stream.read()

table_lines = []
results = {}
benches = []

def filter_empty(var):
    return len(var) != 0

lines = output.split("\n")
for line in lines:
    if len(line) == 0:
        continue

    words = line.split(" ")
    words = list(filter(filter_empty, words))
    
    name_split = words[0].split("_")
    benchname = name_split[0]
    libname = "_".join(name_split[1:])
    timings = " ".join(words[2:])
    
    if not benchname in benches:
        benches.append(benchname)

    if not libname in results:
        results[libname] = {}
    results[libname][benchname] = timings

for libname in results:
    print(libname)
    line = "|" + libname + "|"
    for bench in benches:
        libbenches = results[libname]
        if bench in libbenches:
            line += libbenches[bench]
        line += "|"
        
    table_lines.append(line)

header = "|Library|"
for bench in benches:
    header += bench + "|"
print(header)
header = "|-|"
for bench in benches:
    header += "-|"
print(header)

for line in table_lines:
    print(line)