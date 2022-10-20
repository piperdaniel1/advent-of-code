with open("input.txt") as f:
    lines = f.readlines()

lines = [x[:-1] for x in lines]

counts = [0 for _ in range(len(lines[0]))]

for line in lines:
    for index, char in enumerate(line):
        if char == "0":
            counts[index] -= 1
        else:
            counts[index] += 1

gamma_rate = ""
epsilon_rate = ""

for count in counts:
    if count > 0:
        gamma_rate += "1"
        epsilon_rate += "0"
    else:
        gamma_rate += "0"
        epsilon_rate += "1"

print(f"{gamma_rate=}")
print(f"{epsilon_rate=}")

gamma_rate = int(gamma_rate, 2)
epsilon_rate = int(epsilon_rate, 2)

print(f"{gamma_rate=}")
print(f"{epsilon_rate=}")
print(f"{gamma_rate*epsilon_rate=}")



