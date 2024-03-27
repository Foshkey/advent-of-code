namespace Day5;

public record Row(
    long DestinationStart,
    long SourceStart,
    long Length
);

public record Map
{
    public string? Name { get; set; }
    public List<Row> Rows { get; set; } = new List<Row>();
}

public record Almanac
{
    public List<long> Seeds { get; set; } = new List<long>();
    public List<Map> Maps { get; set; } = new List<Map>();
}

public record Range(long Start, long Length);