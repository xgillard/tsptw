#! /opt/miniconda3/bin/python3

TEMPLATE = """
#[test]
fn {}() {{
    assert_eq!({}, solve("{}"));
}}
"""

def mk_test(name, sol):
   return TEMPLATE.format(name.lower().replace(".", "_"), sol, name)


with open("sol_solomon_potvin_bengio.txt") as f:
    for line in f.readlines():
        toks = line.split()
        if len(toks) >= 2: 
            print(mk_test(toks[0], toks[1]))
