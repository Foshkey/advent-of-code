from unittest import TestCase

from main import get_summary, parse_input, read_from_file
from part_two import find_smudged_reflection


class PartTwoTests(TestCase):
    def test_example(self):
        example_patterns = parse_input(read_from_file("test/example.txt"))
        reflections = [find_smudged_reflection(pattern) for pattern in example_patterns]
        result = get_summary(reflections)
        self.assertEqual(400, result)