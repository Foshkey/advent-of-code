using System.Text.RegularExpressions;
using Microsoft.Win32.SafeHandles;

namespace Day3;

public static partial class Analyzer
{
    [GeneratedRegex("\\d+")]
    private static partial Regex NumberRegex();

    [GeneratedRegex("[^\\d\\.]")]
    private static partial Regex SymbolRegex();

    private static bool IsValid(Match match, List<string> lines, int lineNumber)
    {
        var line = lines[lineNumber];
        var index = match.Index;
        var number = match.Groups[0].Value;

        // Check left
        if (index > 0 && line[index - 1] != '.')
        {
            return true;
        }

        // Check right
        if (index + number.Length < line.Length && line[index + number.Length] != '.')
        {
            return true;
        }

        // Get indexes ready for up & down checks
        var symbolRegex = SymbolRegex();
        var begin = index == 0 ? 0 : index - 1;
        var end = index + number.Length == line.Length
            ? index + number.Length
            : index + number.Length + 1;
        var length = end - begin;

        // Check up
        if (lineNumber > 0)
        {
            var substring = lines[lineNumber - 1].Substring(begin, length);
            if (symbolRegex.IsMatch(substring))
            {
                return true;
            }
        }

        // Check down
        if (lineNumber < lines.Count - 1)
        {
            var substring = lines[lineNumber + 1].Substring(begin, length);
            if (symbolRegex.IsMatch(substring))
            {
                return true;
            }
        }

        return false;
    }

    public static int SummarizeAdjacentNumbers(string fileName)
    {
        var summary = 0;
        var numberRegex = NumberRegex();

        var lines = FileLoader.LoadFile(fileName);
        for (var lineNumber = 0; lineNumber < lines.Count; lineNumber++)
        {
            summary += numberRegex.Matches(lines[lineNumber])
                .Where(match => IsValid(match, lines, lineNumber))
                .Select(match => int.Parse(match.Groups[0].Value))
                .Sum();
        }

        return summary;
    }

    private static List<int> AllIndexesOfGears(this string line)
    {
        List<int> indexes = new();
        for (int index = 0;; index += 1) {
            index = line.IndexOf('*', index);
            if (index == -1)
                return indexes;
            indexes.Add(index);
        }
    }

    private static bool WithinRange(this Match match, int index)
    {
        var number = match.Groups[0].Value;
        return match.Index >= index - number.Length &&
            match.Index + number.Length <= index + number.Length + 1;
    }

    private static int GetGearRatio(int gearIndex, List<string> lines, int lineNumber)
    {
        var adjacentNumbers = new List<int>();
        var numberRegex = NumberRegex();
        
        // Check same line
        adjacentNumbers.AddRange(
            numberRegex.Matches(lines[lineNumber])
            .Where(match => match.WithinRange(gearIndex))
            .Select(match => int.Parse(match.Groups[0].Value))
        );

        // Check above
        if (lineNumber > 0)
        {
            adjacentNumbers.AddRange(
                numberRegex.Matches(lines[lineNumber - 1])
                .Where(match => match.WithinRange(gearIndex))
                .Select(match => int.Parse(match.Groups[0].Value))
            );
        }

        // Check below
        if (lineNumber < lines.Count - 1)
        {
            adjacentNumbers.AddRange(
                numberRegex.Matches(lines[lineNumber + 1])
                .Where(match => match.WithinRange(gearIndex))
                .Select(match => int.Parse(match.Groups[0].Value))
            );
        }

        if (adjacentNumbers.Count != 2)
        {
            // Invalid, return 0
            return 0;
        }

        return adjacentNumbers[0] * adjacentNumbers[1];
    }

    public static int SummarizeGearRatios(string fileName)
    {
        var summary = 0;
        var lines = FileLoader.LoadFile(fileName);
        for (var lineNumber = 0; lineNumber < lines.Count; lineNumber++)
        {
            summary += lines[lineNumber]
                .AllIndexesOfGears()
                .Select(gearIndex => GetGearRatio(gearIndex, lines, lineNumber))
                .Sum();
        }
        return summary;
    }
}