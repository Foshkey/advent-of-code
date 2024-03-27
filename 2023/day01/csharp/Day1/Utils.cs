using System.Collections.Immutable;
using System.Data;
using System.Text.RegularExpressions;

namespace Day1;

public static partial class Utils
{
    [GeneratedRegex("\\d", RegexOptions.Compiled)]
    private static partial Regex DigitRegex();

    private static Dictionary<string, string> SpelledOutMapping = new()
    {
        ["one"] = "1",
        ["two"] = "2",
        ["three"] = "3",
        ["four"] = "4",
        ["five"] = "5",
        ["six"] = "6",
        ["seven"] = "7",
        ["eight"] = "8",
        ["nine"] = "9",
    };

    public static string GetFirstDigit(string input)
    {
        var match = DigitRegex().Match(input);

        if (!match.Success || match.Length < 1)
        {
            throw new ArgumentException("There are no digits!");
        }
        
        return match.Groups[0].Value;
    }

    public static string GetLastDigit(string input)
        => GetFirstDigit(new string(input.Reverse().ToArray()));

    public static int GetCalibrationValue(string input)
    {
        var result = int.Parse(GetFirstDigit(input) + GetLastDigit(input));
        return result;
    }
    
    private static string InsertOccurance(string input, Position position, Dictionary<int, KeyValuePair<string, string>> matches)
    {
        var sortedMatches = matches.ToImmutableSortedDictionary();
        var match = position == Position.First ? sortedMatches.First() : sortedMatches.Last();
        var index = match.Key;
        var map = match.Value;
        return input.Insert(index + 1, map.Value);
    }

    public static string SanitizeInput(string input, Position position = Position.First)
    {
        // First find all occurances
        var matches = new Dictionary<int, KeyValuePair<string, string>>();
        foreach (var map in SpelledOutMapping)
        {
            var indexes = Regex.Matches(input, map.Key)
                .Select(match => match.Index);
            
            foreach (var index in indexes)
            {
                matches[index] = map;
            }
        }

        // If no matches then input is sanitized
        if (matches.Count == 0)
        {
            return input;
        }

        // Replace occurance
        var newInput = InsertOccurance(input, position, matches);

        // If we're on the first then run it again for the last
        if (position == Position.First)
        {
            newInput = SanitizeInput(newInput, position: Position.Last);
        }

        return newInput;
    }
    
    public static List<string> LoadFile(string fileName)
    {
        using var sr = new StreamReader(fileName);
        var list = new List<string>();
        while (true)
        {
            var line = sr.ReadLine();
            if (line == null) 
            {
                break;
            }

            list.Add(line);
        }
        return list;
    }

    public static int LoadCalibrationValue(string fileName)
    {
        var lines = LoadFile(fileName);
        return lines.Sum(GetCalibrationValue);
    }

    public static int LoadSpelledOutCalibrationValue(string fileName)
    {
        var lines = LoadFile(fileName);
        return lines.Sum(line => GetCalibrationValue(SanitizeInput(line)));
    }
}