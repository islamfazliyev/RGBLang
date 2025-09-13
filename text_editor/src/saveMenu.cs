using Raylib_cs;

class saveMenu
{
    private textEdit edit;

    public saveMenu(textEdit edit)
    {
        this.edit = edit;
    }

    public void InitSaveMenu()
    {
        if (Raylib.IsKeyPressed(KeyboardKey.Enter))
        {
            SaveFile();
        }
        Raylib.DrawRectangle(200, 300, 300, 80, Color.Green);
        Raylib.DrawText("Dosya ismi: " + edit.textBufferSaveMenu.ToString(), 210, 320, 20, Color.Black);
    }

    void SaveFile()
    {
        File.WriteAllText($"{edit.textBufferSaveMenu}.rgb", edit.textBuffer.ToString());
        Console.WriteLine($"Saved as {edit.textBufferSaveMenu}.rgb");
    }
}
