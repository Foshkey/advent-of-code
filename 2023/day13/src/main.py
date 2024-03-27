Pattern = list[str]

class Reflection():
    def __init__(self, is_vertical, index, distance = 0, pattern = None):
        self.is_vertical = is_vertical
        self.index = index
        self.distance = distance
        self.pattern = pattern
    def __eq__(self, __value: object) -> bool:
        return self.is_vertical == __value.is_vertical \
            and self.index == __value.index

def read_from_file(file_name: str) -> list[str]:
    with open(file_name, "r") as file:
        return file.readlines()

def parse_input(lines: list[str]) -> list[Pattern]:
    patterns: list[Pattern] = []
    curr_pattern: Pattern = []

    for line in lines:
        line = line.strip("\n")

        # If a blank line, add pattern and reset
        if len(line) == 0:
            patterns.append(curr_pattern.copy())
            curr_pattern = []
            continue

        curr_pattern.append(line)
    
    # Make sure the last pattern is included
    patterns.append(curr_pattern)

    return patterns

def rotate(pattern: Pattern) -> Pattern:
    new_pattern = []
    # loop through the length of the first row
    for x in range(len(pattern[0])):
        # new row becomes the column in reverse order
        new_row = ""
        for y in range(len(pattern) - 1, -1, -1):
            new_row += pattern[y][x]
        new_pattern.append(new_row)
    return new_pattern

def find_possible_reflections(pattern: Pattern, rotated = False) -> list[Reflection]:
    # loop through each row, finding out if it's a possible reflection
    possible_reflections = []
    for row_index in range(0, len(pattern)):
        
        # init, assume it's a reflection
        distance = 0
        is_reflection = False

        # loop inside out, comparing reflection index to corresponding index
        # E.g. if row is 2, then compare 3 to 2, 4 to 1, 5 to 0.
        # if any don't match, then it's not a reflection
        for ref_index in range(row_index + 1, len(pattern)):
            distance = ref_index - row_index
            corr_index = row_index - (distance - 1)
            if corr_index < 0:
                # corresponding row is out of bounds, break out
                break
            if pattern[ref_index] == pattern[corr_index]:
                is_reflection = True
            else:
                is_reflection = False
                break
        
        # if it's a reflection, then add it to the list
        if is_reflection:
            possible_reflections.append(Reflection(rotated, row_index + 1, distance, pattern))
    
    # find possible reflections in the rotated version as well
    if not rotated:
        possible_reflections += find_possible_reflections(rotate(pattern), rotated=True)

    return possible_reflections

def find_best_reflection(possible_reflections: list[Reflection]) -> Reflection:
    if len(possible_reflections) == 0:
        raise Exception("No reflections found!")
    
    # find the best reflection by distance
    best_reflection = possible_reflections[0]
    for reflection in possible_reflections:
        if reflection.distance > best_reflection.distance:
            best_reflection = reflection
    return best_reflection

def find_reflection(pattern: Pattern) -> Reflection:
    # get all possible reflections from original and rotated (for both rows and columns)
    possible_reflections = find_possible_reflections(pattern)
    return find_best_reflection(possible_reflections)

def get_summary(reflections: list[Reflection]) -> int:
    return sum([
        reflection.index * (1 if reflection.is_vertical else 100)
        for reflection in reflections
    ])

if __name__ == "__main__":
    lines = read_from_file("13/input.txt")
    patterns = parse_input(lines)
    reflections = [find_reflection(pattern) for pattern in patterns]
    print(get_summary(reflections))