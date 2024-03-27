namespace Day2;

public static class Analyzer
{
    public static readonly Grouping Cutoff = new()
    {
        Red = 12,
        Green = 13,
        Blue = 14
    };

    public static IEnumerable<Game> GetPossibleGames(string fileName)
    {
        var lines = FileLoader.LoadFile(fileName);
        var games = lines.Select(Parser.ParseLine);
        var impossibleGames = games.Where(game =>
            game.Highest.Red <= Cutoff.Red &&
            game.Highest.Green <= Cutoff.Green &&
            game.Highest.Blue <= Cutoff.Blue
        );
        return impossibleGames;
    }

    public static int SummarizePossibleGames(string fileName)
        => GetPossibleGames(fileName).Select(x => x.GameNumber).Sum();
    
    public static int GetGamePower(Game game)
        => game.Highest.Red * game.Highest.Green * game.Highest.Blue;
    
    public static int SummarizeGamePowers(string fileName)
    {
        var lines = FileLoader.LoadFile(fileName);
        var games = lines.Select(Parser.ParseLine);
        var summary = games.Select(GetGamePower).Sum();
        return summary;
    }
}