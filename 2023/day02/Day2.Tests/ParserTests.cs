namespace Day2.Tests;

public class ParserTests
{
    [Theory]
    [InlineData("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1, 4, 2, 6)]
    [InlineData("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2, 1, 3, 4)]
    [InlineData("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 3, 20, 13, 6)]
    [InlineData("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 4, 14, 3, 15)]
    [InlineData("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5, 6, 3, 2)]
    public void Test_Parser_ParseLine(string input, int expectedGame, int expectedRed, int expectedGreen, int expectedBlue)
    {
        var result = Parser.ParseLine(input);

        Assert.Equal(expectedGame, result.GameNumber);
        Assert.Equal(expectedRed, result.Highest.Red);
        Assert.Equal(expectedGreen, result.Highest.Green);
        Assert.Equal(expectedBlue, result.Highest.Blue);
    }
}