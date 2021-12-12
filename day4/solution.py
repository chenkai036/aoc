import pandas
import numpy


N = 5


def read_input(filepath):
    with open(filepath, 'r') as infile:
        numbers = [int(v) for v in infile.readline().strip().split(',')]
        boards = pandas.read_csv(infile, header=None, sep=' ', skipinitialspace=True)
    boards = [df.reset_index(drop=True) for _, df in boards.groupby(boards.index // N)]
    return numbers, boards


def solution2(data):
    draws, boards = data
    dframe = pandas.concat(boards + [d.T for d in boards], ignore_index=True)
    index = pandas.DataFrame(False, index=dframe.index, columns=dframe.columns)
    for number in draws:
        index = index | (dframe == number)
        aggregated = index.all(axis=1)
        if numpy.any(aggregated):
            row = aggregated[aggregated == True].index
            board_id = (row.values[0] % len(boards)) // N
            marked_index = index.loc[board_id * N: board_id * N + 4, :].reset_index(drop=True)
            unmarked = boards[board_id] * ~marked_index
            return number * unmarked.values.sum()


def solution1(data):
    draws, boards = data
    dframe = pandas.concat(boards + [d.T for d in boards], ignore_index=True)
    index = pandas.DataFrame(False, index=dframe.index, columns=dframe.columns)
    for number in draws:
        index = index | (dframe == number)
        aggregated = index.all(axis=1)
        if numpy.any(aggregated):
            row = aggregated[aggregated == True].index
            board_id = (row.values[0] % len(boards)) // N
            marked_index = index.loc[board_id * N: board_id * N + 4, :].reset_index(drop=True)
            unmarked = boards[board_id] * ~marked_index
            return number * unmarked.values.sum()


if __name__ == '__main__':
    data = read_input('input.txt')
    print(solution1(data))
    print(solution2(data))

