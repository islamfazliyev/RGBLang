using System.Text;
using Raylib_cs;

class textEdit
{
    public StringBuilder textBuffer = new StringBuilder();
    public StringBuilder textBufferSaveMenu = new StringBuilder();
    public StringBuilder textBufferRunMenu = new StringBuilder();

    public void Editor(bool saveMenuActive, bool runMenuActive)
    {
        int key = Raylib.GetCharPressed();
        if (key > 0)
        {
            if (saveMenuActive)
                textBufferSaveMenu.Append((char)key);
            else if (runMenuActive)
                textBufferRunMenu.Append((char)key);
            else
                textBuffer.Append((char)key);
        }

        if (Raylib.IsKeyPressed(KeyboardKey.Backspace))
        {
            if (!saveMenuActive && textBuffer.Length > 0)
                textBuffer.Remove(textBuffer.Length - 1, 1);

            if (saveMenuActive && textBufferSaveMenu.Length > 0)
                textBufferSaveMenu.Remove(textBufferSaveMenu.Length - 1, 1);

            if (runMenuActive && textBufferRunMenu.Length > 0)
                textBufferRunMenu.Remove(textBufferRunMenu.Length - 1, 1);
        }
    }
}
