# This script is written by Farooq Karimi Zadeh to 
# extract hashtag content characters per UAX31.
# A hashtag content character is either from XID_Continue
# also known as XIDC, Emoji character or Extended_Pictographics
# also known as ExtPict. Additionally, '+', '-' and '_' count, too.

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
    if c.attrib.get("XIDC") == "Y":
        return True
    if c.attrib.get("Emoji") == "Y":
        return True
    if c.attrib.get("ExtPict") == "Y":
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
