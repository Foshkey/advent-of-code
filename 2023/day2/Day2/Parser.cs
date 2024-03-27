using System.Text.RegularExpressions;

namespace Day2;

public static partial class Parser
{

    [GeneratedRegex("Game (\\d+)")]
    private static partial Regex GameNumberRegex();

    [GeneratedRegex("(\\d+) red")]
    private static partial Regex RedRegex();

    [GeneratedRegex("(\\d+) green")]
    private static partial Regex GreenRegex();

    [GeneratedRegex("(\\d+) blue")]
    private static partial Regex BlueRegex();

    private static int GetHighestValue(this Regex regex, string input)
        => regex.Matches(input)
            .Select(match => int.Parse(match.Groups[1].Value))
            .Max();

    public static Game ParseLine(string input) => new()
    {
        GameNumber = int.Parse(GameNumberRegex().Match(input).Groups[1].Value),
        Highest = new()
        {
            Red = RedRegex().GetHighestValue(input),
            Green = GreenRegex().GetHighestValue(input),
            Blue = BlueRegex().GetHighestValue(input),
        }
    };
}