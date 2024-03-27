namespace Day4;

public record Card(
    int CardNumber,
    List<int> WinningNumbers,
    List<int> Numbers)
{
    public int Copies { get; set; } = 1;
}