import pandas


def solution1(series: pandas.DataFrame) -> int:
    increases = (series < series.shift(-1))
    return int(increases.sum())


def solution2(series: pandas.DataFrame) -> int:
    rolling_series = series.rolling(window=3).sum()
    return solution1(rolling_series)


if __name__ == '__main__':
    data = pandas.read_csv('input.txt', header=None)
    print(solution1(data))
    print(solution2(data))

