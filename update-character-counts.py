import re

out = b""

with open("README.md", 'rb') as f:
    categories = {}
    total = 0
    last_category_total = 0
    last_category = None
    for line in f:
        out+=line

        if line.startswith(b"## "):
            if last_category is not None:
                categories[last_category] = last_category_total
            last_category = line[3:].split(b'(')[0].strip()
            last_category_total = 0

        elif line.startswith(b"* "):
            total+=1
            last_category_total+=1


    if last_category is not None:
        categories[last_category] = last_category_total

print(categories)
# exit()

with open("README.md", "wb") as f:
    for line in out.split(b"\n"):
        f.write(line)
        if line.startswith(b"## "):
            if re.search(b' *\\(', line):
                f.seek(re.search(b' *\\(', line).span()[0]-len(line), 1)
            f.write(b' ('+str(categories[line[3:].split(b'(')[0].strip()]).encode('utf-8')+b")")
        f.write(b'\n')

print("Total commands: ", total)

