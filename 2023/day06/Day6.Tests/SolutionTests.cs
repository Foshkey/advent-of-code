namespace Day6.Tests;

public class SolutionTests
{
    [Fact]
    public void Example_GetNumberOfWaysToWin()
    {
        var result = Solution.GetNumberOfWaysToWin("example.txt");
        Assert.Equal(288, result);
    }

    [Fact]
    public void Example_GetNumberOfWaysToWin_SingleNumber()
    {
        var result = Solution.GetNumberOfWaysToWin("example.txt", singleRace: true);
        Assert.Equal(71503, result);
    }
}