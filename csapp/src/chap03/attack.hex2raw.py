import sys, os, ast, struct, re

def mainfun() -> None:
    assert len(sys.argv) == 2
    ifname = sys.argv[1]
    assert os.path.isfile(ifname), f"{ifname}"
    assert ifname.endswith(".in"), f"{ifname}"
    ofname = ifname.removesuffix(".in") + ".out"
    
    content = open(ifname, "r").read()
    without_comment = ""
    while len(content) > 0:
        idx = content.find("/*")
        if idx >= 0:
            without_comment += content[:idx]
            content = content[idx:][2:]
            ndx = content.find("*/")
            assert ndx >= 0, "unmatched comment found"
            content = content[ndx:][2:]
        else:
            without_comment += content
            content = ""
    content = without_comment

    with open(ofname, "wb") as of:
        for segment in content.split():
            # print(segment)
            if segment.startswith("0x") and len(segment) > 2:
                num = ast.literal_eval(segment)
                assert isinstance(num, int)
                of.write(struct.pack("<I", num))
            elif len(segment) == 2:
                num = ast.literal_eval(f"0x{segment}")
                of.write(struct.pack("B", num))
            elif segment.startswith("\"") and segment.endswith("\""):
                for ch in segment.removesuffix('"').removeprefix('"'):
                    of.write(struct.pack("B", ord(ch)))
                # of.write(struct.pack("B", 0))
            else:
                raise RuntimeError(f"unknown format for {segment!r}")

if __name__ == "__main__":
    mainfun()


