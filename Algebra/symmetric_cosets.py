from itertools import permutations
from collections import defaultdict


class Permutation:
    def __init__(self, data):
        self.data = data

    def __hash__(self):
        return hash(tuple(self.data))

    def cycles(self):
        placed = set()

        cycles = []

        while len(placed) < len(self.data):
            smallest_unplaced = sorted(set(self.data) - placed)[0]
            cursor = smallest_unplaced
            current_cycle = []

            while not current_cycle or cursor != current_cycle[0]:
                current_cycle.append(cursor)
                placed.add(cursor)
                cursor = self.data[cursor - 1]
            else:
                cycles.append(current_cycle)

                smallest_unplaced = sorted(set(self.data) - placed)
                cursor = smallest_unplaced
                current_cycle = []

        return cycles

    def __repr__(self):
        cycles = self.cycles()
        if all(len(cycle) == 1 for cycle in cycles):
            return "1"

        def print_cycle(cycle):
            return "(" + " ".join(str(element) for element in cycle) + ")"

        return "".join(print_cycle(cycle) for cycle in cycles if len(cycle) != 1)

    def __eq__(self, other):
        return self.data == other.data

    def __lt__(self, other):
        return self.data < other.data

    def __mul__(self, other):
        return Permutation([other.data[a - 1] for a in self.data])


if __name__ == "__main__":
    h = [
        Permutation([1, 2, 3, 4]),
        Permutation([2, 1, 4, 3]),
        Permutation([3, 4, 1, 2]),
        Permutation([4, 3, 2, 1]),
    ]
    s_4 = [Permutation(p) for p in permutations((1, 2, 3, 4))]

    coset_map = defaultdict(list)

    for element in s_4:
        coset = tuple(sorted(element * h_j for h_j in h))
        coset_map[coset].append(element)

    for coset, elements in coset_map.items():
        print(
            " = ".join(
                [
                    "{" + ", ".join(repr(coset_member) for coset_member in coset) + "}",
                    *[repr(element) + "H" for element in elements],
                ]
            )
        )
