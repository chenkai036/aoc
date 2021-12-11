import pandas


def solution1(data: pandas.Series) -> int:
    series = data.apply(lambda v: int(v, base=2))
    nsize = len(data)
    nbits = data.apply(len).max()
    masks = [1 << (nbits - i - 1) for i in range(nbits)]
    ones_freq = [sum((series & mask) != 0) for mask in masks]
    most_common_bits = ''.join('1' if freq * 2 > nsize else '0' for freq in ones_freq)
    gamma = int(most_common_bits, base=2)
    least_common_bits = ''.join('0' if freq * 2 > nsize else '1' for freq in ones_freq)
    epsilon = int(least_common_bits, base=2)
    return gamma * epsilon


def solution2(data: pandas.Series) -> int:
    def find_rating(criteria_fn):
        current, bp = series, 0
        while len(current) != 1:
            bits = (current & masks[bp]) != 0
            current = current[bits == criteria_fn(len(bits), sum(bits))]
            bp += 1
        return int(current)

    def mcv(nsize, ones):
        return not (ones * 2 < nsize)

    def lcv(nsize, ones):
        return ones * 2 < nsize

    series = data.apply(lambda v: int(v, base=2))
    nbits = data.apply(len).max()
    masks = [1 << (nbits - i - 1) for i in range(nbits)]
    oxygen = find_rating(mcv)
    co2 = find_rating(lcv)

    return oxygen * co2


if __name__ == '__main__':
    series = pandas.read_csv('input.txt', header=None, squeeze=True, dtype=str)
    print(solution1(series))
    print(solution2(series))

