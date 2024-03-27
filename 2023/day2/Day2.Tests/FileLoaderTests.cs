namespace Day2.Tests;

public class FileLoaderTests
{
    [Fact]
    public void Test_FileLoader_LoadFile()
    {
        var result = FileLoader.LoadFile("example.txt");
        Assert.Collection(result,
            x => Assert.Equal("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", x),
            x => Assert.Equal("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", x),
            x => Assert.Equal("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", x),
            x => Assert.Equal("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", x),
            x => Assert.Equal("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", x)
        );
    }
}