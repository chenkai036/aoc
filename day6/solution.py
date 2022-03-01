import pandas


def next_fish(s):
    new = pandas.Series([8] * s[s == 0].count(), dtype=int)
    old = s - 1
    old[old < 0] = 6
    return old.append(new, ignore_index=True)


