namespace Day4;

public static class Parser
{
    public static Card ParseLine(string line)
    {
        var parts = line.Split(':', '|');

        // Card number
        var cardParts = parts[0].Split(' ').Where(str => !string.IsNullOrEmpty(str)).ToList();
        var cardNumber = int.Parse(cardParts[1]);

        // Winning Numbers
        var winningNumbers = parts[1].Split(' ')
            .Where(str => !string.IsNullOrEmpty(str))
            .Select(int.Parse)
            .ToList();

        // Numbers
        var numbers = parts[2].Split(' ')
            .Where(str => !string.IsNullOrEmpty(str))
            .Select(int.Parse)
            .ToList();

        return new Card(cardNumber, winningNumbers, numbers);
    }
}