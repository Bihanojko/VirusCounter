from random import randint

QUESTION_COUNT = 1001
MAX_CLICK_COUNT = 145
FILENAME = "tests/input_5"

with open(FILENAME, "w") as output_file:
    output_file.write(f"{QUESTION_COUNT}\n")

    for q_id in range(QUESTION_COUNT):
        output_file.write(f"{randint(4, MAX_CLICK_COUNT)}\n")
