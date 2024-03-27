using System.Text.RegularExpressions;

namespace Day5;

public static partial class FileParser
{

    [GeneratedRegex("(.*) map:")]
    private static partial Regex MapNameRegex();

    public static Almanac ParseFile(string fileName)
    {
        using var sr = new StreamReader(fileName);

        var almanac = new Almanac
        {
            // Seeds
            Seeds = sr.ReadLine()?
                .Split(": ")[1]
                .Split(' ')
                .Select(long.Parse)
                .ToList()
                ?? new List<long>()
        };

        // Skip next
        sr.ReadLine();

        var mapNameRegex = MapNameRegex();
        var currentMap = new Map();

        while (true)
        {
            var line = sr.ReadLine();
            if (line == null)
            {
                break;
            }

            // Check if it's a new map
            var match = mapNameRegex.Match(line);
            if (match.Success)
            {
                currentMap.Name = match.Groups[1].Value;
                almanac.Maps.Add(currentMap);
                continue;
            }

            // If empty line then create new currentMap
            if (line == "")
            {
                currentMap = new();
                continue;
            }

            var numbers = line
                .Split(' ')
                .Select(long.Parse)
                .ToList();
            
            var row = new Row(numbers[0], numbers[1], numbers[2]);
            currentMap.Rows.Add(row);
        }

        return almanac;
    }
}