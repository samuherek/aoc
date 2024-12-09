
INPUT_1 = "(())"
expected = 0

INPUT_2 = "(()(()("
expected = 3

INPUT_3 = "))((((("
expected = 3

INPUT_4 = "))("
expected = -1

def calc(input):
    input = input.read().strip()
    val = 0
    for c in input:
        if c == "(":
            val = val + 1
        elif c == ")":
            val = val - 1
    return val

with open("input.txt") as file:
    print(calc(file))


