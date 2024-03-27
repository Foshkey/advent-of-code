namespace Day1.Tests;

public class UtilsTests
{
    [Theory]
    [InlineData("1abc2", "1")]
    [InlineData("pqr3stu8vwx", "3")]
    [InlineData("a1b2c3d4e5f", "1")]
    [InlineData("treb7uchet", "7")]
    public void Test_GetFirstDigit(string input, string expectedResult)
    {
        Assert.Equal(expectedResult, Utils.GetFirstDigit(input));
    }

    [Fact]
    public void Test_GetFirstDigit_Exception()
    {
        Assert.Throws<ArgumentException>(() => Utils.GetFirstDigit("thisstringhasnodigits"));
    }

    [Theory]
    [InlineData("1abc2", "2")]
    [InlineData("pqr3stu8vwx", "8")]
    [InlineData("a1b2c3d4e5f", "5")]
    [InlineData("treb7uchet", "7")]
    public void Test_GetLastDigit(string input, string expectedResult)
    {
        Assert.Equal(expectedResult, Utils.GetLastDigit(input));
    }

    [Fact]
    public void Test_LoadFile()
    {
        var result = Utils.LoadFile("example.txt");
        Assert.Collection(result,
            x => Assert.Equal("1abc2", x),
            x => Assert.Equal("pqr3stu8vwx", x),
            x => Assert.Equal("a1b2c3d4e5f", x),
            x => Assert.Equal("treb7uchet", x)
        );
    }

    [Theory]
    [InlineData("1abc2", 12)]
    [InlineData("pqr3stu8vwx", 38)]
    [InlineData("a1b2c3d4e5f", 15)]
    [InlineData("treb7uchet", 77)]
    public void Test_GetCalibrationValue(string input, int expectedResult)
    {
        Assert.Equal(expectedResult, Utils.GetCalibrationValue(input));
    }

    [Theory]
    [InlineData("two1nine", "t2wo1n9ine")]
    [InlineData("eightwothree", "e8ightwot3hree")]
    [InlineData("abcone2threexyz", "abco1ne2t3hreexyz")]
    [InlineData("xtwone3four", "xt2wone3f4our")]
    [InlineData("4nineeightseven2", "4n9ineeights7even2")]
    [InlineData("zoneight234", "zo1ne8ight234")]
    [InlineData("7pqrstsixteen", "7pqrsts6ixteen")]
    [InlineData("eightwoeightwo", "e8ightwoeight2wo")]
    [InlineData("9fgsixzkbscvbxdsfive6spjfhzxbzvgbvrthreeoneightn", "9fgs6ixzkbscvbxdsfive6spjfhzxbzvgbvrthreeone8ightn")]
    public void Test_SanitizeInput(string input, string expectedResult)
    {
        Assert.Equal(expectedResult, Utils.SanitizeInput(input));
    }

    [Fact]
    public void Test_LoadCalibrationValue()
    {
        Assert.Equal(142, Utils.LoadCalibrationValue("example.txt"));
    }

    [Fact]
    public void Test_LoadSpelledOutCalibrationValue()
    {
        Assert.Equal(281, Utils.LoadSpelledOutCalibrationValue("example2.txt"));
    }
}