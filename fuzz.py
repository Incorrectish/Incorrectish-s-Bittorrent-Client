#!/usr/bin/env python3
import random, subprocess
from tqdm import tqdm, trange

# string fuzz tests


def gen_bencode_string(n):
    # Generate a random ASCII character for each position in the string
    random_string = "".join([chr(random.randint(32, 126)) for _ in range(n)])
    return f"{n}:{random_string}"


TEST_NUMBER = 10


def fuzz_strings():
    # String tests
    for i in trange(TEST_NUMBER):
        x = random.randint(3, 10000)
        format_string = gen_bencode_string(x)
        # command = f'cargo run -- decode "{format_string}"'
        command = ["cargo", "run", "--", "decode", format_string]
        result = subprocess.run(command, shell=True, stdout=subprocess.PIPE, text=True)
        exit_code = result.returncode
        if exit_code != 0:
            print(f"Failed test on input: `{format_string}`")
            print(result.stdout)
            exit(1)


def gen_bencode_int():
    # Generate a random 64-bit integer
    return f"i{random.randint(-(2**63), (2**63) - 1)}e"


def fuzz_ints():
    # String tests
    for i in trange(TEST_NUMBER):
        format_string = gen_bencode_int()
        # command = f'cargo run -- decode "{format_string}"'
        command = ["cargo", "run", "--", "decode", format_string]
        result = subprocess.run(command, shell=True, stdout=subprocess.PIPE, text=True)
        exit_code = result.returncode
        if exit_code != 0:
            print(f"Failed test on input: `{format_string}`")
            print(result.stdout)
            exit(1)


def gen_bencode_list(elements):
    return f"l{''.join(elements)}e"


def fuzz_lists(max_depth, max_length):
    def generate_element(depth):
        if depth <= 0 or random.random() < 0.2:
            return (
                gen_bencode_int()
                if random.random() < 0.5
                else gen_bencode_string(random.randint(3, 100))
            )
        else:
            return generate_structure(depth - 1)

    def generate_structure(depth):
        return gen_bencode_list(
            [generate_element(depth) for _ in range(random.randint(0, max_length))]
        )

    for _ in trange(TEST_NUMBER):
        depth = random.randint(0, max_depth)
        format_string = generate_structure(depth)
        if _ % 100 == 0:
            print(f"\n\n{format_string}\n\n")

        command = ["cargo", "run", "--", "decode", format_string]
        result = subprocess.run(
            command,
            cwd="/Users/ishan/Code/bit_torrent_client/src",
            shell=True,
            stdout=subprocess.PIPE,
            text=True,
        )
        print(result.stdout)
        exit_code = result.returncode
        if exit_code != 0:
            print(f"Failed test on input: `{format_string}`")
            exit(1)


if __name__ == "__main__":
    fuzz_lists(30, 3)
    print("All tests completed successfully!")
