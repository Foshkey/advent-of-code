namespace Day4;

public static class Analyzer
{
    public static int SummarizeWinningNumbers(string fileName)
    {
        var lines = FileLoader.LoadFile(fileName);
        var cards = lines.Select(Parser.ParseLine);
        var summary = cards
            .Select(card => card.Numbers.Intersect(card.WinningNumbers).Count())
            .Where(num => num > 0)
            .Select(numWonNumbers => Math.Pow(2, numWonNumbers - 1))
            .Sum();
        return (int)summary;
    }

    public static int SummarizeCopies(string fileName)
    {
        var lines = FileLoader.LoadFile(fileName);
        var cards = lines.Select(Parser.ParseLine).ToList();

        for (var cardIndex = 0; cardIndex < cards.Count; cardIndex++)
        {
            var card = cards[cardIndex];
            var wins = card.Numbers.Intersect(card.WinningNumbers).Count();
            for (var winIndex = cardIndex + 1; winIndex <= cardIndex + wins; winIndex++)
            {
                cards[winIndex].Copies += card.Copies;
            }
        }

        return cards.Select(card => card.Copies).Sum();
    }
}