import pymupdf
from sys import argv

doc = pymupdf.open(argv[1])
for page in doc:
    text: bytes = page.get_text().encode("utf8")
    print(text.decode())
    print("-- page --")
