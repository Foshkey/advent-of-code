namespace Day6;

public static class FileParser
{
    private static IEnumerable<int> GetNumbers(string line)
        => line
        .Split(':')[1]
        .Split(' ', StringSplitOptions.RemoveEmptyEntries)
        .Select(int.Parse);

    public static IEnumerable<Race> GetRaceData(string fileName)
    {
        using var reader = new StreamReader(fileName);
        var raceData = new List<Race>();

        var times = GetNumbers(reader.ReadLine() ?? ":");
        var distances = GetNumbers(reader.ReadLine() ?? ":").ToList();

        return times.Select((time, index) => new Race(time, distances[index]));
    }

    private static long GetSingleNumber(string line)
        => long.Parse(line.Replace(" ", "").Split(':')[1]);
        

    public static Race GetSingleRace(string fileName)
    {
        using var reader = new StreamReader(fileName);

        var time = GetSingleNumber(reader.ReadLine() ?? ":0");
        var distance = GetSingleNumber(reader.ReadLine() ?? ":0");

        return new Race(time, distance);
    }
}