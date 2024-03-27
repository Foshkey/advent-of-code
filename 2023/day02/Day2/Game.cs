namespace Day2;

public record Game
{
    public int GameNumber { get; set; }
    public Grouping Highest { get; set; } = new();
}