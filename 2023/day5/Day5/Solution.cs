using System.Diagnostics;

namespace Day5;

public static class Solution
{
    private static long ConvertPosition(this Row row, long position)
        => row.DestinationStart + position - row.SourceStart;

    private static long GetSeedLocation(this List<Map> maps, long seed)
    {
        var position = seed;

        // Assume maps are in order, run through each one sequentially
        foreach (var map in maps)
        {
            // Find a corresponding row
            foreach (var row in map.Rows)
            {
                // If found, adjust position and break out
                if (position >= row.SourceStart && position < row.SourceStart + row.Length)
                {
                    position = row.ConvertPosition(position);
                    break;
                }
            }
        }

        return position;
    }

    public static long GetLowestLocation(string fileName)
    {
        var almanac = FileParser.ParseFile(fileName);
        return almanac.Seeds
            .Select(almanac.Maps.GetSeedLocation)
            .Aggregate((a, b) => a < b ? a : b);
    }

    public static IEnumerable<Range> MapRange(this Map map, Range range)
    {
        foreach (var row in map.Rows)
        {
            // Entire range falls within the map
            if (range.Start >= row.SourceStart && range.Start + range.Length <= row.SourceStart + row.Length)
            {
                yield return new Range(row.ConvertPosition(range.Start), range.Length);

                range = new Range(range.Start, 0);
                break;
            }

            // Beginning of range overlaps
            if (range.Start >= row.SourceStart && range.Start < row.SourceStart + row.Length)
            {
                var newStart = row.ConvertPosition(range.Start);
                var newLength = range.Start - row.SourceStart;
                yield return new Range(newStart, newLength);

                range = new Range(range.Start + newLength, range.Length - newLength);
                continue;
            }

            // End of range overlaps
            if (range.Start + range.Length >= row.SourceStart && range.Start + range.Length < row.SourceStart + row.Length)
            {
                var newStart = row.DestinationStart;
                var newLength = range.Start + range.Length - row.SourceStart;
                yield return new Range(newStart, newLength);

                range = new Range(range.Start, range.Length - newLength);
                continue;
            }

            // Entire map falls within range
            if (range.Start <= row.SourceStart && range.Start + range.Length >= row.SourceStart + row.Length)
            {
                var beforeRangeStart = range.Start;
                var beforeRangeLength = row.SourceStart - range.Start;
                var beforeMappedRanges = map.MapRange(new Range(beforeRangeStart, beforeRangeLength));
                foreach (var beforeRange in beforeMappedRanges)
                {
                    yield return beforeRange;
                }

                yield return new Range(row.DestinationStart, row.Length);

                range = new Range(row.SourceStart + row.Length, range.Start + range.Length - (row.SourceStart + row.Length));
                continue;
            }
        }

        // Account for any unmapped
        if (range.Length > 0)
        {
            yield return range;
        }
    }

    public static IEnumerable<Range> GetSeedLocationRanges(this IEnumerable<Map> maps, IEnumerable<Range> seedRanges)
    {
        IEnumerable<Range> currentRanges = seedRanges;
        foreach (var map in maps)
        {
            currentRanges = currentRanges.SelectMany(map.MapRange);
        }
        return currentRanges;
    }

    public static long GetLowestLocationSeedRange(string fileName)
    {
        var almanac = FileParser.ParseFile(fileName);
        var seedStart = (long)-1;
        var ranges = new List<Range>();
        
        foreach (var seedNumber in almanac.Seeds)
        {
            // Check if this is the start index
            if (seedStart == -1)
            {
                seedStart = seedNumber;
                continue;
            }

            // Add range to list
            ranges.Add(new Range(seedStart, seedNumber));

            // Reset for next range
            seedStart = -1;
        }

        var mappedRanges = almanac.Maps.GetSeedLocationRanges(ranges);
        var lowest = mappedRanges.Select(r => r.Start).Aggregate((a, b) => a < b ? a : b);
        return lowest;
    }
}