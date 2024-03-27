namespace Day2.Tests;

public class AnalyzerTests
{
    [Fact]
    public void Test_SummarizePossibleGames()
    {
        var result = Analyzer.SummarizePossibleGames("example.txt");
        Assert.Equal(8, result);
    }

    [Fact]
    public void Test_SummarizeGamePowers()
    {
        var result = Analyzer.SummarizeGamePowers("example.txt");
        Assert.Equal(2286, result);
    }
}