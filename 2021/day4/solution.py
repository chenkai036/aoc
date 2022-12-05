import pandas
import numpy


N = 5


def read_input(filepath):
    with open(filepath, 'r') as infile:
        numbers = [int(v) for v in infile.readline().strip().split(',')]
        boards = pandas.read_csv(infile, header=None, sep=' ', skipinitialspace=True)
    boards = [df.reset_index(drop=True) for _, df in boards.groupby(boards.index // N)]
    return numbers, boards


def solution(draws, boards, win_value):
    dframe = pandas.concat(boards + [d.T for d in boards], ignore_index=True)
    prev_index = pandas.Int64Index([])
    for i, n in enumerate(draws):
        numbers = draws[:i + 1]
        matched_rc = dframe.isin(numbers).all(axis=1)
        matched_board = matched_rc.groupby(matched_rc.index // N).any()
        matched_board = matched_board.groupby(matched_board.index % len(boards)).any()
        current_index = matched_board[matched_board].index
        if len(current_index) == win_value and len(prev_index) == win_value - 1:
            board = boards[int(current_index.difference(prev_index).values)]
            unmarked_board = board * ~board.isin(numbers)
            return n * unmarked_board.values.sum()
        else:
            prev_index = current_index


if __name__ == '__main__':
    draws, boards = read_input('input.txt')
    print(solution(draws, boards, 1))
    print(solution(draws, boards, len(boards)))


