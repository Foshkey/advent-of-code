namespace Day4.Tests;

public class AnalyzerTests
{
    [Fact]
    public void Analyzer_SummarizeWinningNumbers()
    {
        var result = Analyzer.SummarizeWinningNumbers("example.txt");
        Assert.Equal(13, result);
    }
    
    [Fact]
    public void Analyzer_SummarizeCopies()
    {
        var result = Analyzer.SummarizeCopies("example.txt");
        Assert.Equal(30, result);
    }
}