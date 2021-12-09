import pandas


def solution1(data: pandas.DataFrame) -> int:
    forward = data[data.command == 'forward'].units.sum()
    down = data[data.command == 'down'].units.sum()
    up = data[data.command == 'up'].units.sum()
    return forward * (down - up)


def solution2(data: pandas.DataFrame) -> int:
    forward, depth, aim = 0, 0, 0
    for _, row in data.iterrows():
        if row.command == 'down':
            aim += row.units
        elif row.command == 'up':
            aim -= row.units
        else:
            forward += row.units
            depth += row.units * aim
    return forward * depth


if __name__ == '__main__':
    data = pandas.read_csv('input.txt', header=None, names=['command', 'units'], sep=' ')
    print(solution1(data))
    print(solution2(data))

