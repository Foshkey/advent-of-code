namespace Day4;

public static class FileLoader
{
    public static List<string> LoadFile(string fileName)
    {
        using var sr = new StreamReader(fileName);
        var list = new List<string>();
        while (true)
        {
            var line = sr.ReadLine();
            if (line == null) 
            {
                break;
            }

            list.Add(line);
        }
        return list;
    }
}