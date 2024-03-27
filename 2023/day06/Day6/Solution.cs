namespace Day6;

public static class Solution
{
    private static long DistanceTraveled(long timeButtonHeld, long totalTime)
    {
        var speed = timeButtonHeld; // millimeter per millisecond
        var remainingTime = totalTime - timeButtonHeld;
        return speed * remainingTime;
    }

    private static long CalculateWaysToWin(Race race)
    {
        var sum = (long)0;

        for (var i = (long)1; i < race.Time; i++)
        {
            if (DistanceTraveled(i, race.Time) > race.MinDistance)
            {
                sum++;
            }
        }

        return sum;
    }

    public static long GetNumberOfWaysToWin(string fileName, bool singleRace = false)
    {
        if (singleRace)
        {
            var race = FileParser.GetSingleRace(fileName);
            return CalculateWaysToWin(race);
        }
        
        var raceData = FileParser.GetRaceData(fileName);
        return raceData.Aggregate((long)1, (acc, race) => acc * CalculateWaysToWin(race));
    }
}