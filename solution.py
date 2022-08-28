import linecache 
 
max_clicks_count = 1_000
# max_clicks_count = 10_000_000_000
input_filename = "tests/input_1"


def generate_virus_counts(max_clicks_count):
    copies_count_beginning = [1, 2, 3, 5]
    virus_counts = [0]

    f = open("virus_counts_1k.txt", "w")
    f.write("0\n")

    for idx, copy_count in enumerate(copies_count_beginning):
        virus_counts.append(copy_count + virus_counts[idx])
        f.write(f"{virus_counts[-1]}\n")

    previous_copies_counts = copies_count_beginning[-3:]

    for idx in range(0, max_clicks_count - len(virus_counts) + 1):
        virus_counts.append(virus_counts[-1] + previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2])
        previous_copies_counts[idx % 3] = previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]
        f.write(f"{virus_counts[-1]}\n")
        # f.write(f"{virus_counts[-1] % 1_000_000_007}\n")

    # f = open("virus_counts.txt", "w")
    # for elem in virus_counts:
    #     f.write(f"{elem % 1_000_000_007}\n")
    f.close()


def generate_virus_counts_2(n, virus_counts={0: 0, 1: 1, 2: 3, 3: 6, 4: 11}, previous_copies_counts=[2, 3, 5]):
    if n not in virus_counts:
        length = len(virus_counts)
        numbers_to_compute = n - length + 1

        for i in range(numbers_to_compute):
            virus_counts[length + i] = virus_counts[length + i - 1] + previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]
            previous_copies_counts[(length + i + 1) % 3] = previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]

    return virus_counts, previous_copies_counts


def get_virus_counts(input_filename):
    with open(input_filename, "r") as input_file:
        input_content = input_file.read().splitlines()
        # check that first row is a number in a given range, check number of rows is as expected

    # V1
    # for x in input_content[1:]:
    #     line = linecache.getline("virus_counts.txt", int(x)).strip()
    #     print(line)
    
    # V2
    # virus_counts, previous_copies_counts = generate_virus_counts_2(int(input_content[1].strip()))
    # print(virus_counts[int(input_content[1].strip())])

    # for x in input_content[2:]:
    #     virus_counts, previous_copies_counts = generate_virus_counts_2(int(x), virus_counts, previous_copies_counts)
    #     print(virus_counts[int(x)] % 1_000_000_007)

    # V3
    wanted_counts = [int(x) for x in input_content[1:]]
    answers = {}
    max_count = max(wanted_counts)

    virus_count_prev = 11
    previous_copies_counts = [2, 3, 5]

    for i in range(5, max_count + 1):
        virus_count = virus_count_prev + previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]
        previous_copies_counts[(i + 4) % 3] = previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]
        virus_count_prev = virus_count

        if i in wanted_counts:
            answers[i] = virus_count % 1_000_000_007
    
    for x in wanted_counts:
        print(answers[x])


# generate_virus_counts(max_clicks_count)

get_virus_counts(input_filename)
