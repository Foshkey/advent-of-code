namespace Day5.Tests;

public class SolutionTests
{
    [Fact]
    public void Solution_GetLowestLocation()
    {
        var result = Solution.GetLowestLocation("example.txt");
        Assert.Equal(35, result);
    }

    [Fact]
    public void Solution_GetLowestLocationSeedRange()
    {
        var result = Solution.GetLowestLocationSeedRange("example.txt");
        Assert.Equal(46, result);
    }

    [Fact]
    public void Solution_MapRange()
    {
        var range = new Range(45, 11);
        var map = new Map()
        {
            Rows = new()
            {
                new Row(52, 50, 48)
            }
        };

        var result = map.MapRange(range);

        Assert.Collection(
            result,
            x => Assert.Equal(new Range(52, 6), x),
            x => Assert.Equal(new Range(45, 5), x)
        );
    }
}