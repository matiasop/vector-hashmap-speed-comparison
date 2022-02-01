from random import randint
from time import time


def vector_benchmark(vector, index):
    initial_time = time()
    for item in vector:
        if item == index:
            result = item
    total_time = time() - initial_time
    return total_time


def hashmap_benchmark(hashmap, key):
    initial_time = time()
    result = hashmap[key]
    total_time = time() - initial_time
    return total_time


def benchmark(length, index):
    vector = [i for i in range(length)]
    hashmap = {i: i for i in range(length)}
    return vector_benchmark(vector, index), hashmap_benchmark(hashmap, index)


def main():
    lengths = [10, 100, 1_000, 10_000, 100_000, 1_000_000]
    samples = 100

    for length in lengths:
        vector_benchmarks = []
        hashmap_benchmarks = []
        for _ in range(samples):
            index = randint(0, length - 1)
            vector_time, hashmap_time = benchmark(length, index)
            vector_benchmarks.append(vector_time)
            hashmap_benchmarks.append(hashmap_time)
        vector_average_time = sum(vector_benchmarks)/samples
        hashmap_average_time = sum(hashmap_benchmarks)/samples

        print(f"Array/HashMap Length: {length}, vector average time: {vector_average_time}, hashmap average time: {hashmap_average_time}")


if __name__ == "__main__":
    main()
