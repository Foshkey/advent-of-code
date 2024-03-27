from unittest import TestCase

from main import Reflection, find_reflection, get_summary, parse_input, read_from_file, rotate


class MainTests(TestCase):

    def test_parse_input(self):
        input = [
            "#..#",
            ".#.#",
            "",
            "#.",
            ".#"
        ]

        result = parse_input(input)

        self.assertEqual([
            [
                "#..#",
                ".#.#"
            ],
            [
                "#.",
                ".#"
            ]
        ], result)

    def test_rotate(self):
        pattern = [
            "#..##",
            "..#.#",
            ".#.#."
        ]

        result = rotate(pattern)

        self.assertEqual([
            "..#",
            "#..",
            ".#.",
            "#.#",
            ".##",
        ], result)

    def test_find_reflection(self):
        example_patterns = parse_input(read_from_file("test/example.txt"))
        
        reflection1 = find_reflection(example_patterns[0])
        reflection2 = find_reflection(example_patterns[1])

        self.assertTrue(reflection1.is_vertical)
        self.assertEqual(5, reflection1.index)
        self.assertFalse(reflection2.is_vertical)
        self.assertEqual(4, reflection2.index)

    def test_get_summary(self):
        reflection1 = Reflection(True, 2)
        reflection2 = Reflection(False, 3)

        result = get_summary([reflection1, reflection2])

        self.assertEqual(302, result)

    def test_example(self):
        example_patterns = parse_input(read_from_file("test/example.txt"))
        reflections = [find_reflection(pattern) for pattern in example_patterns]
        result = get_summary(reflections)
        self.assertEqual(405, result)