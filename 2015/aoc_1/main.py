
INPUT_1 = "(())"
expected = 0

INPUT_2 = "(()(()("
expected = 3

INPUT_3 = "))((((("
expected = 3

INPUT_4 = "))("
expected = -1

INPUT_5 = "()())"
expected = 5

def part1(input):
    input = input.read().strip()
    val = 0
    for c in input:
        if c == "(":
            val = val + 1
        elif c == ")":
            val = val - 1
    return val

def part2(input):
    val = 0
    result = 0;
    for i in range(len(input)):
        if input[i] == "(":
            val = val + 1
        elif input[i] == ")":
            val = val - 1
        if val < 0:
            result = i + 1
            break;

    return result

with open("input.txt") as file:
    input = file.read().strip()
    print(part2(input))


