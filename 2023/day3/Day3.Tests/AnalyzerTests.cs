namespace Day3.Tests;

public class AnalyzerTests
{
    [Fact]
    public void Analyzer_SummarizeAdjacentNumbers()
    {
        var result = Analyzer.SummarizeAdjacentNumbers("example.txt");
        Assert.Equal(4361, result);
    }

    [Fact]
    public void Analyzer_SummarizeGearRatios()
    {
        var result = Analyzer.SummarizeGearRatios("example.txt");
        Assert.Equal(467835, result);
    }
}