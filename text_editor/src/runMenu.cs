using Raylib_cs;

class runMenu
{
    private textEdit edit;

    public runMenu(textEdit edit)
    {
        this.edit = edit;
    }

    public void InitRunMenu()
    {
        if (Raylib.IsKeyPressed(KeyboardKey.Enter))
        {
            RunFile();
        }
        Raylib.DrawRectangle(200, 300, 300, 80, Color.Green);
        Raylib.DrawText("Filename: " + edit.textBufferRunMenu.ToString(), 210, 320, 20, Color.Black);
    }

    void RunFile()
    {
        var psi = new System.Diagnostics.ProcessStartInfo
        {
            FileName = "./../target/debug/RGB-Interpreter",
            Arguments = $"--file {edit.textBufferRunMenu}.rgb",
            RedirectStandardOutput = true,
            UseShellExecute = false,
        };
        var process = System.Diagnostics.Process.Start(psi);
        var output = process.StandardOutput.ReadToEnd();
        Console.WriteLine(psi.Arguments);
        Console.WriteLine(output);
        Console.WriteLine(edit.textBufferRunMenu.ToString());
    }
}
