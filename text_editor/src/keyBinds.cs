using Raylib_cs;

class keyBinds
{
    public bool saveMenuActive = false;
    public bool runMenuActive = false;

    public void Update()
    {
        if (Raylib.IsKeyDown(KeyboardKey.LeftControl))
        {
            if (Raylib.IsKeyPressed(KeyboardKey.S))
            {
                saveMenuActive = true;
            }

            if (Raylib.IsKeyPressed(KeyboardKey.R))
            {
                runMenuActive = true;
            }
        }
    }
}
