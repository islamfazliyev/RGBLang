using Raylib_cs;

class initWindow
{
    public void init()
    {
        Raylib.InitWindow(600, 600, "text editor");
        Raylib.SetTargetFPS(60);

        textEdit edit = new textEdit();
        keyBinds keyBinds = new keyBinds();
        saveMenu save = new saveMenu(edit);
        runMenu run = new runMenu(edit);

        while (!Raylib.WindowShouldClose())
        {
            edit.Editor(keyBinds.saveMenuActive, keyBinds.runMenuActive);
            keyBinds.Update();

            Raylib.BeginDrawing();
            Raylib.ClearBackground(Color.Black);

            if (keyBinds.saveMenuActive)
            {
                save.InitSaveMenu();
            }

            if (keyBinds.runMenuActive)
            {
                run.InitRunMenu();
            }

            int x = 0;
            int y = 40;
            int charWidth = 15;

            foreach (char c in edit.textBuffer.ToString())
            {
                Color color = c == '0' ? Color.Red : c == '1' ? Color.Green : c == '2' ? Color.Blue : Color.Orange;
                Raylib.DrawText(c.ToString(), x, y, 20, color);
                x += charWidth;
                if (x + charWidth > 600)
                {
                    x = 0;
                    y += 30;
                }
            }

            Raylib.EndDrawing();
        }
    }
}
