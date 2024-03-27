from main import Pattern, Reflection, find_best_reflection, find_possible_reflections, find_reflection, get_summary, parse_input, read_from_file

def filter_reflections(possible_reflections, original_reflection) -> list[Reflection]:
    new_list = []
    for reflection in possible_reflections:
        if reflection != original_reflection:
            new_list.append(reflection)
    return new_list

def change_symbol(pattern: Pattern, x: int, y: int) -> Pattern:
    new_pattern = []
    new_char = "#" if pattern[x][y] == "." else "."
    for i, row in enumerate(pattern):
        # if it's the index we want to change then splice together a new row
        new_row = row[:y] + new_char + row[y+1:] if i == x else row
        new_pattern.append(new_row)
    return new_pattern

def find_smudged_reflection(pattern: Pattern) -> Reflection:
    possible_reflections = []
    
    # loop through each symbol, changing it and adding possible reflections to the list
    for x in range(len(pattern)):
        for y in range(len(pattern[0])):
            new_pattern = change_symbol(pattern, x, y)
            possible_reflections += find_possible_reflections(new_pattern)

    # and filter out any that match the original reflection
    possible_reflections = filter_reflections(possible_reflections, find_reflection(pattern))

    # finally find the best reflection
    return find_best_reflection(possible_reflections)

if __name__ == "__main__":
    lines = read_from_file("13/input.txt")
    patterns = parse_input(lines)
    reflections = [find_smudged_reflection(pattern) for pattern in patterns]
    print(get_summary(reflections))