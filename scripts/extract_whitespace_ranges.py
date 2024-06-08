# This script is written by Farooq Karimi Zadeh to 
# extract hashtag whitespace characters. A whitespace character is within Zs category

from xml.etree import ElementTree
from dataclasses import dataclass
from pprint import pprint

@dataclass
class Range:
    start: int
    end: int

    def __repr__(self) -> str:
        start = hex(self.start)
        end = hex(self.end)
        return f"{start}..={end},"

def predicate(c) -> bool:
    if c.attrib.get("gc") == "Zs":
        return True
    return False

if __name__ == "__main__":
    data = ""
    with open("ucd.all.flat.xml") as fp:
        data = fp.read()

    tree = ElementTree.fromstring(data)
    repertoire = list(tree.iter())[2]
    accepted_chars = map(lambda c: int(c.get("cp", "0"), 16), filter(predicate, repertoire.iter()))
    accepted_chars = sorted(accepted_chars)
    ranges: list[Range] = list()
    
    r: Range | None = None
    for c in accepted_chars:
        if r is None:
            r = Range(c, c)
        elif c == (r.end + 1):
            r.end = c
        elif c <= r.end:
            continue
        else:
            ranges.append(r)
            r = Range(c, c)
    
    if r:
        ranges.append(r)
    pprint(ranges)
